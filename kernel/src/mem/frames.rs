use core::slice::from_raw_parts_mut;
use lazy_static::lazy_static;
use spin::Mutex;
use macros::os_test;

use crate::BootData;
use crate::multiboot::{MemoryKind, MemoryMapPointer};
use crate::mem::address::PhysicalAddress;
use crate::mem::KiB;

const FRAME_SIZE: usize = 4096;

lazy_static! {
    pub static ref FRAME_MAP: Mutex<FrameMap> = Mutex::new(FrameMap { total_frames: 0, frames: &mut [] });
}

#[derive(PartialEq, Clone, Copy)]
#[repr(usize)]
pub enum FrameSize {
    SMALL = FRAME_SIZE,
    LARGE = FRAME_SIZE * FRAME_SIZE,
    HUGE = FRAME_SIZE * FRAME_SIZE * FRAME_SIZE,
}

pub struct Frame {
    pub start_address: PhysicalAddress,
    pub free: bool,
    pub size: FrameSize,
}

impl Frame {
    pub fn for_address(address: PhysicalAddress) -> Frame {
        let frame = address.data() >> 12; // shift away address offset
        let byte = frame / 8;
        let offset = frame % 8;
        Frame {
            start_address: PhysicalAddress::new(frame << 12),
            free: FRAME_MAP.lock().frames[byte as usize] & (0x1 << offset) == 0,
            size: FrameSize::SMALL,
        }
    }
}

pub struct FrameMap {
    total_frames: usize,
    frames: &'static mut [u8],
}

// TODO: optimize setting blocks
impl FrameMap {
    // TODO: setup paging as needed for frame map
    pub unsafe fn init(&mut self, boot_data: &BootData) {
        let start_address = PhysicalAddress::new((boot_data.kernel_end / FRAME_SIZE + 1) * FRAME_SIZE);

        crate::logln!("[frames] Creating frame map starting at 0x{:X}.", start_address.data());

        self.create_buffer(start_address, boot_data.mb_info.memory_map());

        let mut current = Some(boot_data.mb_info.memory_map());
        loop {
            match current {
                Some(pointer) => {
                    if { pointer.entry.kind } == MemoryKind::Usable {
                        let mut first = pointer.entry.base as usize / FRAME_SIZE;
                        if pointer.entry.base as usize % FRAME_SIZE != 0 {
                            first += 1
                        }
                        let mut count = pointer.entry.limit as usize / FRAME_SIZE;
                        if pointer.entry.limit as usize % FRAME_SIZE != 0 {
                            count += 1;
                        }

                        for i in 0..count.max(self.frames.len()) {
                            self.set_frame(i + first, true);
                        }
                    }
                    current = pointer.next();
                }
                None => break
            }
        }

        // mark everything until end of frame map as used
        let frames_used = self.frames.len() / FRAME_SIZE + 1;
        let frame_index = (self as *const FrameMap as usize) / FRAME_SIZE;
        let last_frame = frame_index + frames_used + 1;
        for i in 0..last_frame {
            self.set_frame(i, false);
        }

        crate::logln!(
            "[frames] Created frame map for {} frames ({} KiBs).",
            self.total_frames,
            self.total_frames * FRAME_SIZE / KiB,
        );
    }

    unsafe fn create_buffer(&mut self, start_address: PhysicalAddress, memory_map: MemoryMapPointer) {
        let mut last_address: usize = 0;

        let mut current = Some(memory_map);
        loop {
            match current {
                Some(pointer) => {
                    if { pointer.entry.kind } == MemoryKind::Usable {
                        last_address = (pointer.entry.base + pointer.entry.limit) as usize
                    }
                    current = pointer.next();
                }
                None => break
            }
        }

        self.total_frames = last_address / FRAME_SIZE + usize::from(last_address % FRAME_SIZE != 0);
        self.frames = from_raw_parts_mut(
            start_address.data() as *mut u8,
            self.total_frames / 8 + usize::from(self.total_frames % FRAME_SIZE != 0),
        );

        self.frames.fill(u8::MAX);
    }

    fn set_frame(&mut self, index: usize, free: bool) {
        assert!(index <= self.total_frames, "Frame outside expected range.");
        let byte = index / 8;
        let offset = index % 8;
        if free {
            self.frames[byte] = self.frames[byte] & ((0x1 << offset) ^ 0xFF)
        } else {
            self.frames[byte] = self.frames[byte] | (0x1 << offset)
        }
    }

    // TODO: make this something resembling performant
    // TODO: allow collecting multiple frames
    pub fn alloc_free(&mut self) -> Frame {
        let mut index = 0;
        while self.frames[index] == u8::MAX {
            index += 1
        }
        let frame = index * 8 + self.frames[index].trailing_ones() as usize;
        self.set_frame(frame, false);

        crate::logln!("[frames] Allocated frame {} at address 0x{:X}.", frame, frame << 12);

        Frame {
            start_address: PhysicalAddress::new(frame << 12),
            free: false,
            size: FrameSize::SMALL,
        }
    }
}

#[os_test]
fn mem_frames_set_index() {
    // this is fine since the first MiB shouldn't be used anyway
    for i in 0..16 {
        FRAME_MAP.lock().set_frame(i, true);
        assert_eq!(Frame::for_address(PhysicalAddress::new(i << 12)).free, true)
    }

    for i in 0..16 {
        FRAME_MAP.lock().set_frame(i, false);
        assert_eq!(Frame::for_address(PhysicalAddress::new(i << 12)).free, false)
    }
}

#[os_test]
fn mem_frames_alloc_free() {
    let mut index = 0;

    while !(Frame::for_address(PhysicalAddress::new(index << 12)).free) {
        index += 1;
    }

    let alloc_frame = FRAME_MAP.lock().alloc_free();
    assert_eq!(
        alloc_frame.start_address.data(),
        index << 12,
    );

    assert!(!Frame::for_address(alloc_frame.start_address).free)
}
