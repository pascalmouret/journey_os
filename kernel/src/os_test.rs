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
}
