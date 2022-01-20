#[cfg(test)]
pub mod os_test {
    use crate::io::port::Port;
    use crate::io::stdout::STD_OUT;
    use crate::io::serial::COM1;

    const ISA_PORT: u16 = 0xF4;
    const SUCCESS_CODE: u32 = 42;
    const FAILURE_CODE: u32 = 1;

    pub struct OSTest<'test> {
        pub name: &'test str,
        pub test: &'test dyn Fn(),
    }

    pub fn test_runner(tests: &[&OSTest]) {
        STD_OUT.lock().set(&COM1);
        crate::println!("Running tests...");
        for test in tests {
            crate::print!("{}...", test.name);
            (test.test)();
            crate::print!("ok\n")
        }
        exit(true);
    }

    pub fn test_panic() {
        exit(false);
    }

    fn exit(success: bool) {
        let port = unsafe { Port::open(ISA_PORT) };
        port.write(if success { SUCCESS_CODE } else { FAILURE_CODE });
    }
}

#[cfg(test)]
pub use crate::os_test::os_test::{*};
