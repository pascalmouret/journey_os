#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
use core::fmt::Write;
use crate::multiboot::{MemoryMapEntry, MB_MAGIC_ADDR};

mod vga;
mod multiboot;
mod mem;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    write!(vga::CONSOLE.lock(), "{}", info).unwrap();
    loop {};
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let magic = unsafe { *(MB_MAGIC_ADDR as *const u32) };

    if magic != 0x2BADB002 {
        write!(vga::CONSOLE.lock(), "Magic number does not match. Expected: 0x2BADB002, Found: {:X}", magic).unwrap();
    }

    let mut current = unsafe { Some(multiboot::MultibootInfo::memory_map()) };
    loop {
        match current {
            Some(entry) => {
                write!(vga::CONSOLE.lock(), "{:X} {:X} {:X} {:?}\n", entry.size, entry.base, entry.limit, entry.kind);
                current = unsafe { entry.next() };
            }
            None => break
        }
    }

    if mem::frames::FRAME_MAP.lock().index_is_free(1) {
        write!(vga::CONSOLE.lock(), "Free\n");
    } else {
        write!(vga::CONSOLE.lock(), "Used\n");
    }

    loop {}
}
