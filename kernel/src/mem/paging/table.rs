use core::marker::PhantomData;
use crate::mem::paging::entry::Entry;
use crate::mem::address::{PhysicalAddress};
use crate::mem::frames::{FRAME_MAP};

const ENTRY_COUNT: usize = 512;

pub trait TableLevel {}

pub enum Level4 {}
pub enum Level3 {}
pub enum Level2 {}
pub enum Level1 {}

impl TableLevel for Level4 {}
impl TableLevel for Level3 {}
impl TableLevel for Level2 {}
impl TableLevel for Level1 {}

pub trait HierarchicalLevel: TableLevel {
    type NextLevel: TableLevel;
}

impl HierarchicalLevel for Level4 {
    type NextLevel = Level3;
}

impl HierarchicalLevel for Level3 {
    type NextLevel = Level2;
}

impl HierarchicalLevel for Level2 {
    type NextLevel = Level1;
}

#[repr(C, align(4096))]
pub struct Table<L: TableLevel> {
    entries: [Entry; ENTRY_COUNT],
    level: PhantomData<L>,
}

impl<L: TableLevel> Table<L> {
    pub fn get_entry(&self, index: usize) -> &Entry {
        &self.entries[index]
    }

    pub fn set(&mut self, index: usize, address: &PhysicalAddress, is_page: bool) {
        self.entries[index].set(address, is_page)
    }
}

impl <L: HierarchicalLevel> Table<L> {
    pub fn create_next(&mut self, index: usize) -> &mut Table<L::NextLevel> {
        let frame = FRAME_MAP.lock().alloc_free();
        let ptr = frame.start_address.data() as *mut [u64; ENTRY_COUNT];
        self.entries[index].set(&frame.start_address, false);

        unsafe {
            ptr.as_mut().unwrap().fill(0);
            (ptr as *mut Table<L::NextLevel>).as_mut().unwrap()
        }
    }

    pub fn get_next(&self, index: usize) -> Option<&Table<L::NextLevel>> {
        self.entries[index]
            .get_target_address()
            .map(|address| unsafe { &*(address.data() as *const Table<L::NextLevel>) })
    }

    pub fn get_next_mut(&mut self, index: usize) -> Option<&mut Table<L::NextLevel>> {
        self.entries[index]
            .get_target_address()
            .map(|address| unsafe { &mut *(address.data() as *mut Table<L::NextLevel>) })
    }

    pub fn get_or_create_next(&mut self, index: usize) -> &mut Table<L::NextLevel> {
        if self.entries[index].is_present() {
            self.get_next_mut(index).unwrap()
        } else {
            self.create_next(index)
        }
    }
}

impl Table<Level1> {
    pub fn get_address(&self, index: usize) -> Option<PhysicalAddress> {
        self.entries[index].get_target_address()
    }
}

impl Table<Level4> {
    pub fn load_current<'table>() -> &'table mut Table<Level4> {
        let mut ptr: *mut Table<Level4>;
        unsafe {
            asm!("mov %cr3, {}", out(reg) ptr, options(att_syntax));
            ptr.as_mut().unwrap()
        }
    }
}
