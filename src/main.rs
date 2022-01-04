#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
use crate::multiboot::MultibootInfo;

mod vga;
mod multiboot;
mod mem;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {};
}

#[repr(C, packed)]
pub struct BootData {
    mb_magic: u32,
    mb_info: &'static MultibootInfo,
    kernel_start: usize,
    kernel_end: usize,
}

#[no_mangle]
pub extern "cdecl" fn kernel_main(boot_data: &BootData) -> ! {
    if boot_data.mb_magic != 0x2BADB002 {
        panic!("Magic number does not match. Expected: 0x2BADB002, Found: {:X}", { boot_data.mb_magic });
    } else {
        println!("Hello Rust!");
    }

    unsafe { mem::frames::FRAME_MAP.lock().init(boot_data) };
    println!("{:X} bytes of memory mapped.", mem::frames::FRAME_MAP.lock().total_memory_bytes());

    if mem::frames::FRAME_MAP.lock().index_is_free(1) {
        println!("Free");
    } else {
        println!("Used");
    }

    loop {}
}
