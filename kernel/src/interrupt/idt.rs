use spin::Mutex;
use lazy_static::lazy_static;

use macros::os_test;
use crate::interrupt::pic::{PIC_PRIMARY, PIC_SECONDARY};
use crate::interrupt::handlers::{test, ExceptionStackFrame};

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
        // disable PIC
        // TODO: remap PIC offsets
        PIC_PRIMARY.lock().disable();
        PIC_SECONDARY.lock().disable();

        self.load_as_idt();
    }

    pub unsafe fn set_interrupt_handler(
        &mut self,
        vector: usize,
        handler: unsafe extern "x86-interrupt" fn(ExceptionStackFrame)
    ) {
        self.entries[vector] = IDTEntry::new(
            handler as *const u8 as usize,
            GateType::Interrupt,
        );
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
        INTERRUPTS.lock().set_interrupt_handler(42, test);
        asm!("int 42")
    }
}
