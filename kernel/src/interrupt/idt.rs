use spin::Mutex;
use lazy_static::lazy_static;

use macros::os_test;
use crate::interrupt::handlers;
use crate::interrupt::handlers::ExceptionStackFrame;

const IDT_SIZE: usize = 256;

// Present bit set, CPU ring 0
const DEFAULT_ATTRIBUTES: u8 = 0x80;
const GDT_CODE: u16 = 8;

lazy_static! {
    pub static ref INTERRUPTS: Mutex<IDT> = Mutex::new(IDT::new());
}

#[repr(C, packed)]
struct IDTDescriptor {
    limit: u16,
    base: u64,
}

#[repr(u8)]
pub enum GateType {
    Interrupt = 0xE,
    Trap = 0xF,
}

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct IDTEntry {
    address_low: u16,
    code_segment: u16,
    ist: u8,
    attributes: u8,
    address_mid: u16,
    address_high: u32,
    reserved: u32,
}

impl IDTEntry {
    pub fn new(address: usize, gate_type: GateType) -> IDTEntry {
        let attributes = DEFAULT_ATTRIBUTES | gate_type as u8;

        IDTEntry {
            ist: 0,
            reserved: 0,
            code_segment: GDT_CODE,
            attributes,
            address_low: address as u16,
            address_mid: (address >> 16) as u16,
            address_high: (address >> 32) as u32,
        }
    }

    pub fn empty() -> IDTEntry {
        IDTEntry {
            ist: 0,
            reserved: 0,
            code_segment: 0,
            attributes: 0,
            address_low: 0,
            address_mid: 0,
            address_high: 0,
        }
    }
}

#[repr(C)]
pub struct IDT {
    entries: [IDTEntry; IDT_SIZE]
}

impl IDT {
    pub fn new() -> IDT {
        IDT { entries: [IDTEntry::empty(); IDT_SIZE] }
    }

    pub unsafe fn init(&mut self) {
        crate::logln!("[interrupts] Initialize IDT.");
        self.set_handlers();
        self.load_as_idt();

        crate::interrupt::pic::PIC::initialize(0x20, 0x28);

        // re-enable maskable interrupts
        asm!("sti");
    }

    unsafe fn set_handlers(&mut self) {
        // CPU EXCEPTIONS
        self.set_exception_handler(0x00, handlers::divide_by_zero);
        self.set_exception_handler(0x01, handlers::debug);
        self.set_exception_handler(0x02, handlers::non_maskable);
        self.set_exception_handler(0x03, handlers::breakpoint);
        self.set_exception_handler(0x04, handlers::overflow);
        self.set_exception_handler(0x05, handlers::bound_range_exceeded);
        self.set_exception_handler(0x06, handlers::invalid_opcode);
        self.set_exception_handler(0x07, handlers::device_not_available);
        self.set_error_exception_handler(0x08, handlers::double_fault);
        self.set_error_exception_handler(0x0A, handlers::invalid_tss);
        self.set_error_exception_handler(0x0B, handlers::segment_not_present);
        self.set_error_exception_handler(0x0C, handlers::stack_segment_fault);
        self.set_error_exception_handler(0x0D, handlers::general_protection_fault);
        self.set_error_exception_handler(0x0E, handlers::page_fault);
        self.set_exception_handler(0x10, handlers::x87_floating_point_exception);
        self.set_error_exception_handler(0x11, handlers::alignment_check);
        self.set_exception_handler(0x12, handlers::machine_check);
        self.set_exception_handler(0x13, handlers::simd_floating_point_exception);
        self.set_exception_handler(0x14, handlers::virtualization_exception);
        self.set_error_exception_handler(0x15, handlers::control_protection_exception);
        self.set_exception_handler(0x1C, handlers::hypervisor_injection_exception);
        self.set_error_exception_handler(0x1D, handlers::vmm_communication_exception);
        self.set_error_exception_handler(0x1E, handlers::security_exception);
        // RESERVED UNTIL 0x1F

        // PIC interrupts
        self.set_interrupt_handler(0x20, handlers::pic_irq_0);
        self.set_interrupt_handler(0x21, handlers::pic_irq_1);
        self.set_interrupt_handler(0x22, handlers::pic_irq_2);
        self.set_interrupt_handler(0x23, handlers::pic_irq_3);
        self.set_interrupt_handler(0x24, handlers::pic_irq_4);
        self.set_interrupt_handler(0x25, handlers::pic_irq_5);
        self.set_interrupt_handler(0x26, handlers::pic_irq_6);
        self.set_interrupt_handler(0x27, handlers::pic_irq_7);
        self.set_interrupt_handler(0x28, handlers::pic_irq_8);
        self.set_interrupt_handler(0x29, handlers::pic_irq_9);
        self.set_interrupt_handler(0x2A, handlers::pic_irq_10);
        self.set_interrupt_handler(0x2B, handlers::pic_irq_11);
        self.set_interrupt_handler(0x2C, handlers::pic_irq_12);
        self.set_interrupt_handler(0x2D, handlers::pic_irq_13);
        self.set_interrupt_handler(0x2E, handlers::pic_irq_14);
        self.set_interrupt_handler(0x2F, handlers::pic_irq_15);
    }

    pub unsafe fn set_exception_handler(
        &mut self,
        vector: usize,
        handler: unsafe extern "x86-interrupt" fn(ExceptionStackFrame),
    ) {
        self.set_handler(
            vector,
            IDTEntry::new(
                handler as *const u8 as usize,
                GateType::Trap,
            ),
        );
    }

    pub unsafe fn set_error_exception_handler(
        &mut self,
        vector: usize,
        handler: unsafe extern "x86-interrupt" fn(ExceptionStackFrame, error: u32),
    ) {
        self.set_handler(
            vector,
            IDTEntry::new(
                handler as *const u8 as usize,
                GateType::Trap,
            ),
        );
    }

    pub unsafe fn set_interrupt_handler(
        &mut self,
        vector: usize,
        handler: unsafe extern "x86-interrupt" fn(ExceptionStackFrame)
    ) {
        self.set_handler(
            vector,
            IDTEntry::new(
                handler as *const u8 as usize,
                GateType::Interrupt,
            ),
        );
    }

    fn set_handler(&mut self, vector: usize, entry: IDTEntry) {
        self.entries[vector] = entry;
    }

    unsafe fn load_as_idt(&self) {
        let descriptor: IDTDescriptor = IDTDescriptor {
            base: self as *const IDT as u64,
            limit: core::mem::size_of::<IDT>() as u16 - 1,
        };
        asm!("lidt ({idt})", idt = in(reg) &descriptor, options(nostack, att_syntax));
    }
}

#[os_test]
fn interrupt_idt_new_idt_entry() {
    let entry = IDTEntry::new(0x123456789ABCDEF1, GateType::Interrupt);

    assert_eq!(entry.address_low, 0xDEF1);
    assert_eq!(entry.address_mid, 0x9ABC);
    assert_eq!(entry.address_high, 0x12345678);
}

#[os_test]
fn interrupt_idt_int() {
    unsafe {
        INTERRUPTS.lock().set_interrupt_handler(42, handlers::test);
        asm!("int 42")
    }
}
