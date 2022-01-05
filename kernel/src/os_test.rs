#[cfg(test)]
pub mod os_test {
    use core::panic::PanicInfo;

    const ISA_PORT: usize = 0xF4;
    const SUCCESS_CODE: u32 = 42;
    const FAILURE_CODE: u32 = 1;

    pub struct OSTest<'test> {
        pub name: &'test str,
        pub test: &'test dyn Fn(),
    }

    pub fn test_runner(tests: &[&OSTest]) {
        crate::println!("Running tests...");
        for test in tests {
            crate::print!("{}...   ", test.name);
            (test.test)();
            crate::print!("[ok]\n")
        }
        exit(true);
    }

    pub fn test_panic(info: &PanicInfo) {
        exit(false);
    }

    fn exit(success: bool) {
        let code = if success { SUCCESS_CODE } else { FAILURE_CODE };
        unsafe {
            asm! {
            "OUT %eax, %dx",
            in("dx") ISA_PORT,
            in("eax") code,
            options(att_syntax),
            }
        }
    }
}

#[cfg(test)]
pub use crate::os_test::os_test::{*};
