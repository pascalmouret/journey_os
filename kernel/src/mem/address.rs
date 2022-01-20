use macros::os_test;

use crate::mem::paging::table::{Level4, Table};

const L1_SHIFT: u8 = 12;
const L2_SHIFT: u8 = 21;
const L3_SHIFT: u8 = 30;
const L4_SHIFT: u8 = 39;
const INDEX_MASK: usize = 0x1FF; // 9 lowest bits


pub struct PhysicalAddress(usize);
pub struct VirtualAddress(usize);

impl PhysicalAddress {
    pub fn new(address: usize) -> PhysicalAddress {
        return PhysicalAddress { 0: address }
    }

    pub fn data(&self) -> usize {
        self.0
    }
}

impl VirtualAddress {
    pub fn new(address: usize) -> VirtualAddress {
        return VirtualAddress { 0: address }
    }

    pub fn data(&self) -> usize {
        self.0
    }

    pub fn l4_index(&self) -> usize { self.0 >> L4_SHIFT & INDEX_MASK }
    pub fn l3_index(&self) -> usize { self.0 >> L3_SHIFT & INDEX_MASK }
    pub fn l2_index(&self) -> usize { self.0 >> L2_SHIFT & INDEX_MASK }
    pub fn l1_index(&self) -> usize { self.0 >> L1_SHIFT & INDEX_MASK }

    // TODO: remove
    unsafe fn to_physical_address(&self, table: &Table<Level4>) -> PhysicalAddress {
        table
            .get_next(self.0 >> 39).unwrap()
            .get_next(self.0 >> 30).unwrap()
            .get_next(self.0 >> 21).unwrap()
            .get_address(self.0 >> 12).unwrap()
    }
}

#[os_test]
fn mem_paging_virtual_address_to_physical_address() {
    let table = unsafe { &*(0x1000 as *const Table<Level4>) };
    let kernel_address = 0x100000;

    assert_eq!(
        unsafe { (VirtualAddress { 0: kernel_address }).to_physical_address(table).0 },
        kernel_address,
    )
}
