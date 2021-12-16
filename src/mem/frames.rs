const PAGE_SIZE: usize = 4096;

use core::slice::from_raw_parts_mut;
use crate::multiboot::{MemoryKind, MemoryMapEntry, MultibootInfo};
use lazy_static::lazy_static;
use spin::Mutex;

use core::fmt::Write;
use crate::vga::CONSOLE;

lazy_static! {
    pub static ref FRAME_MAP: Mutex<FrameMap> = unsafe { Mutex::new(FrameMap::create()) };
}

pub struct FrameMap {
    frames: &'static mut [u8],
}

impl FrameMap {
    unsafe fn create() -> FrameMap {
        write!(CONSOLE.lock(), "Building frame map\n");
        let mmap = MultibootInfo::memory_map();
        let kernel_start: usize;
        let kernel_end: usize;

        // There has to be a better way to do this...
        asm!("mov $KERNEL_START, {}", out(reg) kernel_start, options(att_syntax));
        asm!("mov $KERNEL_END, {}", out(reg) kernel_end, options(att_syntax));

        let aligned = (kernel_end / PAGE_SIZE + 1) * PAGE_SIZE;
        write!(CONSOLE.lock(), "{:X} {:X} {:X}\n", kernel_start, kernel_end, aligned);
        let mut map = FrameMap {
            frames: from_raw_parts_mut(aligned as *mut u8, FrameMap::required_frame_bytes(mmap))
        };

        let mut current = Some(mmap);
        loop {
            match current {
                Some(entry) => {
                    let mut first = entry.base as usize / PAGE_SIZE;
                    if entry.base as usize % PAGE_SIZE != 0 {
                        first += 1
                    }
                    let mut count = entry.limit as usize / PAGE_SIZE;
                    if entry.limit as usize % PAGE_SIZE != 0 {
                        count += 1;
                    }

                    if first > map.frames.len() {
                        break;
                    }

                    write!(CONSOLE.lock(), "{:X} {:X}\n", first, count);
                    for i in 0..count.max(map.frames.len()) {
                        if entry.kind == MemoryKind::Usable {
                            map.free_index(i + first);
                        } else {
                            map.alloc_index(i + first);
                        }
                    }
                    current = entry.next();
                }
                None => break
            }
        }

        let frames_used = map.frames.len() / PAGE_SIZE + 1;
        let frame_index = (&map as *const FrameMap as usize) / PAGE_SIZE;
        for i in 0..frames_used {
            map.alloc_index(i + frame_index);
        }

        let mut kernel_first = kernel_start / PAGE_SIZE;
        if kernel_start % PAGE_SIZE != 0 {
            kernel_first += 1
        }
        let mut kernel_count = kernel_end / PAGE_SIZE;
        if kernel_end % PAGE_SIZE != 0 {
            kernel_count += 1;
        }

        for i in 0..kernel_count {
            map.alloc_index(i + kernel_first)
        }

        return map;
    }

    fn required_frame_bytes(mmap: &MemoryMapEntry) -> usize {
        let mut last_address: usize = 0;

        let mut current = Some(&*mmap);
        loop {
            match current {
                Some(entry) => {
                    if entry.kind == MemoryKind::Usable {
                        last_address = (entry.base + entry.limit) as usize
                    }
                    current = entry.next();
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
}



