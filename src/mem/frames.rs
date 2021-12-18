const PAGE_SIZE: usize = 4096;

use core::slice::from_raw_parts_mut;
use crate::multiboot::{MemoryKind, MemoryMapPointer};
use lazy_static::lazy_static;
use spin::Mutex;
use crate::BootData;

lazy_static! {
    pub static ref FRAME_MAP: Mutex<FrameMap> = unsafe { Mutex::new(FrameMap { frames: &mut [] }) };
}

pub struct FrameMap {
    frames: &'static mut [u8],
}

// TODO: optimize setting blocks
impl FrameMap {
    // TODO: mark final pages as used if they don't exist
    // TODO: mark gaps in map as used
    // TODO: setup paging as needed for frame map
    // TODO: allow only one init
    pub unsafe fn init(&mut self, boot_data: &BootData) {
        self.frames = from_raw_parts_mut(
            ((boot_data.kernel_end / PAGE_SIZE + 1) * PAGE_SIZE) as *mut u8,
            FrameMap::required_frame_bytes(boot_data.mb_info.memory_map()),
        );

        let mut current = Some(boot_data.mb_info.memory_map());
        loop {
            match current {
                Some(pointer) => {
                    let mut first = pointer.entry.base as usize / PAGE_SIZE;
                    if pointer.entry.base as usize % PAGE_SIZE != 0 {
                        first += 1
                    }
                    let mut count = pointer.entry.limit as usize / PAGE_SIZE;
                    if pointer.entry.limit as usize % PAGE_SIZE != 0 {
                        count += 1;
                    }

                    if first > self.frames.len() {
                        break;
                    }

                    for i in 0..count.max(self.frames.len()) {
                        if { pointer.entry.kind } == MemoryKind::Usable {
                            self.free_index(i + first);
                        } else {
                            self.alloc_index(i + first);
                        }
                    }
                    current = pointer.next();
                }
                None => break
            }
        }

        let frames_used = self.frames.len() / PAGE_SIZE + 1;
        let frame_index = (self as *const FrameMap as usize) / PAGE_SIZE;
        for i in 0..frames_used {
            self.alloc_index(i + frame_index);
        }

        let mut kernel_first = boot_data.kernel_start / PAGE_SIZE;
        if boot_data.kernel_start % PAGE_SIZE != 0 {
            kernel_first += 1
        }
        let mut kernel_count = boot_data.kernel_end / PAGE_SIZE;
        if boot_data.kernel_end % PAGE_SIZE != 0 {
            kernel_count += 1;
        }

        for i in 0..kernel_count {
            self.alloc_index(i + kernel_first)
        }
    }

    fn required_frame_bytes(memory_map: MemoryMapPointer) -> usize {
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

        let last_page = last_address / PAGE_SIZE + usize::from(last_address % PAGE_SIZE != 0);
        last_page / 8 + usize::from(last_page % PAGE_SIZE != 0)
    }

    fn free_index(&mut self, frame: usize) {
        let byte = frame / 8;
        let offset = frame % 8;
        self.frames[byte] = self.frames[byte] | (0x1 << offset)
    }

    fn alloc_index(&mut self, frame: usize) {
        let byte = frame / 8;
        let offset = frame % 8;
        self.frames[byte] = self.frames[byte] & ((0x1 << offset) & 0xFF)
    }

    pub fn index_is_free(&self, frame: usize) -> bool {
        let byte = frame / 8;
        let offset = frame % 8;
        self.frames[byte] & ((0x1 << offset) & 0xFF) > 0
    }

    pub fn total_memory_bytes(&self) -> usize {
        self.frames.len() * PAGE_SIZE * 8
    }
}



