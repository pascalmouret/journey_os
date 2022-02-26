#[cfg(test)]
pub mod os_test {
    use crate::io::port::{Port};

    const ISA_PORT: u16 = 0xF4;
    const SUCCESS_CODE: u32 = 42;
    const FAILURE_CODE: u32 = 1;

    pub struct OSTest<'test> {
        pub name: &'test str,
        pub test: &'test dyn Fn(),
    }

    pub fn test_runner(tests: &[&OSTest]) {
        crate::logln!("[os_test] Running tests...");
        for test in tests {
            crate::logln!("[os_test] {}...", test.name);
            (test.test)();
        }
        exit(true);
    }

    pub fn test_panic() {
        exit(false);
    }

    fn exit(success: bool) {
        let port = unsafe { Port::<u32>::open(ISA_PORT) };
        port.write(if success { SUCCESS_CODE } else { FAILURE_CODE });
    }
}

#[cfg(test)]
pub use crate::os_test::os_test::{*};
