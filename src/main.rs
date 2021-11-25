#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::fmt::Write;

mod vga;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!("int 80");
    }

    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    for i in 1..30 {
        write!(vga::CONSOLE.lock(), "Hello {}\n", i);
    }

    loop {}
}
