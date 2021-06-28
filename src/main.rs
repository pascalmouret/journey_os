#![feature(asm)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        asm!("int 80");
    }

    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for i in (0..HELLO.len()) {
        write_char(HELLO[i], i)
    }

    loop {}
}

fn write_char(c: u8, pos: usize) {
    unsafe {
        *(0xb8000 as *mut u8).offset(pos as isize * 2) = c;
    }
}
