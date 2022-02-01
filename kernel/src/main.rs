#![feature(panic_info_message)]
#![no_std]
#![no_main]
#![feature(asm)]
#![feature(global_asm)]
#![feature(custom_test_frameworks)]
#![feature(const_mut_refs)]
#![feature(alloc_error_handler)]
#![test_runner(crate::os_test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use crate::mem::KiB;
use crate::multiboot::MultibootInfo;

#[cfg(test)]
use crate::os_test::test_panic;

mod io;
mod multiboot;
mod mem;
mod util;
mod os_test;

global_asm!(include_str!("boot.s"), options(att_syntax));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    #[cfg(test)]
    test_panic();

    loop {};
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[repr(C, packed)]
pub struct BootData {
    mb_magic: u32,
    mb_info: &'static MultibootInfo,
    kernel_start: usize,
    kernel_end: usize,
}

#[no_mangle]
pub unsafe extern "cdecl" fn kernel_main(boot_data: &BootData) -> ! {
    if boot_data.mb_magic != 0x2BADB002 {
        panic!("Magic number does not match. Expected: 0x2BADB002, Found: {:X}", { boot_data.mb_magic });
    } else {
        println!("Booting Journey OS 0.1.0");
    }

    mem::frames::FRAME_MAP.lock().init(boot_data);
    mem::allocator::ALLOCATOR.lock().init(0x4000_0000_0000, 10 * KiB);

    #[cfg(test)]
    test_main();

    println!("Ready");
    loop {}
}
