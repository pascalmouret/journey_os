#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
use crate::multiboot::MB_MAGIC_ADDR;

mod vga;
mod multiboot;
mod mem;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {};
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let magic = unsafe { *(MB_MAGIC_ADDR as *const u32) };

    if magic != 0x2BADB002 {
        panic!("Magic number does not match. Expected: 0x2BADB002, Found: {:X}", magic);
    } else {
        println!("Hello Rust!");
    }

    println!("{:X} bytes of memory mapped.", mem::frames::FRAME_MAP.lock().total_memory_bytes());

    if mem::frames::FRAME_MAP.lock().index_is_free(1) {
        println!("Free");
    } else {
        println!("Used");
    }

    loop {}
}
