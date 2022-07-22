use core::arch::asm;

use crate::mem::address::{PhysicalAddress, VirtualAddress};
use crate::mem::frames::{Frame, FRAME_MAP};
use crate::mem::paging::mapper::map_frame;
use crate::mem::paging::table::Table;

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
    let address: usize;
    asm!("mov %cr2, {}", out(reg) address, options(att_syntax));

    crate::logln!("Page fault: 0x{:X}. Address: 0x{:X}", error, address);

    // page is not present and we're in kernel mode (bit 1 = present, bit 3: user)
    if error & 0x1 == 0 && error & 0x4 == 0 {
        let frame = &FRAME_MAP.lock().alloc_free();

        map_frame(
            frame,
            &VirtualAddress::new(address >> 12 << 12),
            Table::load_current(),
        )
    } else {
        // TODO: implement userland paging
        asm!("hlt");
    }
}

// 0x10: FAULT
pub unsafe extern "x86-interrupt" fn x87_floating_point_exception(stack_frame: ExceptionStackFrame) {
    crate::logln!("x87 floating point exception. Shrugging.");
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

// 0x20: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_0(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 0");
    crate::interrupt::pic::PIC::end_of_interrupt(0);
}

// 0x21: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_1(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 1");
    crate::interrupt::pic::PIC::end_of_interrupt(1);
}

// 0x22: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_2(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 2");
    crate::interrupt::pic::PIC::end_of_interrupt(2);
}

// 0x23: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_3(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 3");
    crate::interrupt::pic::PIC::end_of_interrupt(3);
}

// 0x24: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_4(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 4");
    crate::interrupt::pic::PIC::end_of_interrupt(4);
}

// 0x25: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_5(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 5");
    crate::interrupt::pic::PIC::end_of_interrupt(5);
}

// 0x26: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_6(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 6");
    crate::interrupt::pic::PIC::end_of_interrupt(6);
}

// 0x27: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_7(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 7");
    crate::interrupt::pic::PIC::end_of_interrupt(7);
}

// 0x28: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_8(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 8");
    crate::interrupt::pic::PIC::end_of_interrupt(8);
}

// 0x29: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_9(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 9");
    crate::interrupt::pic::PIC::end_of_interrupt(9);
}

// 0x2A: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_10(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 10");
    crate::interrupt::pic::PIC::end_of_interrupt(10);
}

// 0x2B: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_11(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 11");
    crate::interrupt::pic::PIC::end_of_interrupt(11);
}

// 0x2C: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_12(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 12");
    crate::interrupt::pic::PIC::end_of_interrupt(12);
}

// 0x2D: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_13(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 13");
    crate::interrupt::pic::PIC::end_of_interrupt(13);
}

// 0x2E: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_14(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 14");
    crate::interrupt::pic::PIC::end_of_interrupt(14);
}

// 0x2F: PIC Interrupt
pub unsafe extern "x86-interrupt" fn pic_irq_15(stack_frame: ExceptionStackFrame) {
    crate::logln!("IRQ 15");
    crate::interrupt::pic::PIC::end_of_interrupt(15);
}
