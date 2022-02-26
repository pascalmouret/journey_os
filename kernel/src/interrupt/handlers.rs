#[repr(C)]
pub struct ExceptionStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

pub unsafe extern "x86-interrupt" fn test(stack_frame: ExceptionStackFrame) {
    crate::logln!("Interrupt called at 0x{:X}", stack_frame.instruction_pointer);
}
