use crate::mem::address::PhysicalAddress;

// Flags are conveniently stored in all the irrelevant parts of the address,
// so we can just "and out" the actual address (bits 12 - 51)
const ADDRESS_MASK: usize = 0xFFFFFFFFFF000;
const PRESENT_FLAG: usize = 0x1; // bit 0
const WRITABLE_FLAG: usize = 0x2; // bit 1
const IS_PAGE_FLAG: usize = 0x80; // bit 7

#[repr(packed(8))]
pub struct Entry(usize);

impl Entry {
    pub fn get_target_address(&self) -> Option<PhysicalAddress> {
        if self.is_present() {
            Some(PhysicalAddress::new(self.0 & ADDRESS_MASK))
        } else {
            None
        }
    }

    pub fn is_present(&self) -> bool {
        self.0 & PRESENT_FLAG > 0
    }

    pub fn set(&mut self, address: &PhysicalAddress, is_page: bool) {
        self.0 = (address.data() & ADDRESS_MASK) + PRESENT_FLAG + WRITABLE_FLAG;
        if is_page {
            self.0 + IS_PAGE_FLAG;
        }
        crate::logln!("Set entry to: 0x{:X}", self.0);
    }
}
