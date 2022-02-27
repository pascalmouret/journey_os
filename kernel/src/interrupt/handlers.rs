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

// 0x00: FAULT
pub unsafe extern "x86-interrupt" fn divide_by_zero(stack_frame: ExceptionStackFrame) {
    crate::logln!("Divide by Zero. Aborting.");
    asm!("hlt");
}

// 0x01: FAULT/TRAP
pub unsafe extern "x86-interrupt" fn debug(stack_frame: ExceptionStackFrame) {
    crate::logln!("Debug exception. Not implemented. Continuing.");
}

// 0x02: INTERRUPT
pub unsafe extern "x86-interrupt" fn non_maskable(stack_frame: ExceptionStackFrame) {
    crate::logln!("Non maskable interrupt. Aborting.");
    asm!("hlt");
}

// 0x03: TRAP
pub unsafe extern "x86-interrupt" fn breakpoint(stack_frame: ExceptionStackFrame) {
    crate::logln!("Breakpoint. Not implemented. Continuing.");
}

// 0x04: TRAP
pub unsafe extern "x86-interrupt" fn overflow(stack_frame: ExceptionStackFrame) {
    crate::logln!("Overflow. Aborting.");
    asm!("hlt");
}

// 0x05: FAULT
pub unsafe extern "x86-interrupt" fn bound_range_exceeded(stack_frame: ExceptionStackFrame) {
    crate::logln!("Bound range exceeded. Aborting.");
    asm!("hlt");
}

// 0x06: FAULT
pub unsafe extern "x86-interrupt" fn invalid_opcode(stack_frame: ExceptionStackFrame) {
    crate::logln!("Invalid opcode. Aborting.");
    asm!("hlt");
}

// 0x07: FAULT
pub unsafe extern "x86-interrupt" fn device_not_available(stack_frame: ExceptionStackFrame) {
    crate::logln!("Device not available. Aborting.");
    asm!("hlt");
}

// 0x08: ABORT
pub unsafe extern "x86-interrupt" fn double_fault(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Double fault: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x0A: FAULT
pub unsafe extern "x86-interrupt" fn invalid_tss(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Invalid TSS: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x0B: FAULT
pub unsafe extern "x86-interrupt" fn segment_not_present(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Segment not present: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x0C: FAULT
pub unsafe extern "x86-interrupt" fn stack_segment_fault(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Stack segment fault: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x0D: FAULT
pub unsafe extern "x86-interrupt" fn general_protection_fault(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("General protection fault: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x0E: FAULT
pub unsafe extern "x86-interrupt" fn page_fault(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Page fault: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x10: FAULT
pub unsafe extern "x86-interrupt" fn x87_floating_point_exception(stack_frame: ExceptionStackFrame) {
    crate::logln!("x87 floating point exception. Aborting.");
    asm!("hlt");
}

// 0x11: FAULT
pub unsafe extern "x86-interrupt" fn alignment_check(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Alignment check: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x12: ABORT
pub unsafe extern "x86-interrupt" fn machine_check(stack_frame: ExceptionStackFrame) {
    crate::logln!("Machine check. Aborting.");
    asm!("hlt");
}

// 0x13: FAULT
pub unsafe extern "x86-interrupt" fn simd_floating_point_exception(stack_frame: ExceptionStackFrame) {
    crate::logln!("SIMD floating point exception. Aborting.");
    asm!("hlt");
}

// 0x14: FAULT
pub unsafe extern "x86-interrupt" fn virtualization_exception(stack_frame: ExceptionStackFrame) {
    crate::logln!("Virtualization exception. Aborting.");
    asm!("hlt");
}

// 0x15: FAULT
pub unsafe extern "x86-interrupt" fn control_protection_exception(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Control protection exception: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x1C: FAULT
pub unsafe extern "x86-interrupt" fn hypervisor_injection_exception(stack_frame: ExceptionStackFrame) {
    crate::logln!("Hypervisor injection exception. Aborting.");
    asm!("hlt");
}

// 0x1D: FAULT
pub unsafe extern "x86-interrupt" fn vmm_communication_exception(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("VMM communication exception: 0x{:X}. Aborting.", error);
    asm!("hlt");
}

// 0x1E: FAULT
pub unsafe extern "x86-interrupt" fn security_exception(stack_frame: ExceptionStackFrame, error: u32) {
    crate::logln!("Security exception: 0x{:X}. Aborting.", error);
    asm!("hlt");
}
