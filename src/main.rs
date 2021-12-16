#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
use core::fmt::Write;
use crate::multiboot::MB_MAGIC_ADDR;

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
        panic!("Magic number does not match. Expected: 0x2BADB002, Found: {:X}", magic);
    } else {
        write!(vga::CONSOLE.lock(), "Hello Rust!\n").unwrap();
    }

    write!(vga::CONSOLE.lock(), "{:X} bytes of memory mapped.\n", mem::frames::FRAME_MAP.lock().total_memory_bytes()).unwrap();

    if mem::frames::FRAME_MAP.lock().index_is_free(1) {
        write!(vga::CONSOLE.lock(), "Free\n").unwrap();
    } else {
        write!(vga::CONSOLE.lock(), "Used\n").unwrap();
    }

    loop {}
}
