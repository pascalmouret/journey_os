use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use macros::os_test;

use crate::mem::address::VirtualAddress;
use crate::mem::frames::FrameSize;
use crate::mem::paging::mapper::map_frame;
use crate::mem::frames::FRAME_MAP;
use crate::mem::align_address;
use crate::mem::paging::table::Table;
use crate::util::locked::Locked;

#[global_allocator]
pub static ALLOCATOR: Locked<LinkedHeap> = Locked::new(LinkedHeap::new());

struct MemoryNode {
    size: usize,
    next: Option<&'static mut MemoryNode>,
}

impl MemoryNode {
    const fn new(size: usize) -> MemoryNode {
        MemoryNode { size, next: None }
    }

    pub fn start_address(&self) -> usize {
        self as *const Self as usize
    }

    pub fn end_address(&self) -> usize {
        self.start_address() + self.size
    }
}

pub struct Region {
    node: &'static MemoryNode,
    alloc_start: usize,
    alloc_end: usize,
}

impl Region {
    fn new(node: &'static MemoryNode, alloc_start: usize, alloc_end: usize) -> Region {
        Region { node, alloc_start, alloc_end }
    }
}

pub struct LinkedHeap {
    list: MemoryNode,
}

impl LinkedHeap {
    pub const fn new() -> LinkedHeap {
        LinkedHeap { list: MemoryNode::new(0) }
    }

    pub unsafe fn init(&mut self, start: usize, size: usize) {
        let paging_table = Table::load_current();
        let frames = size / FrameSize::SMALL as usize + usize::from(size % FrameSize::SMALL as usize != 0);

        // TODO: use page faults to assign pages
        for i in 0..frames {
            let frame = FRAME_MAP.lock().alloc_free();
            map_frame(
                &frame,
                &VirtualAddress::new(start + FrameSize::SMALL as usize * i),
                paging_table,
            );
        }

        self.free_region(start, size);

        crate::logln!("[allocator] Built 0x{:X} byte kernel heap at 0x{:X}.", size, start);
    }

    pub unsafe fn free_region(&mut self, start: usize, size: usize) {
        assert!(size >= core::mem::size_of::<MemoryNode>(), "Freed region too small for memory node.");
        assert_eq!(start % core::mem::align_of::<MemoryNode>(), 0);

        crate::logln!("[allocator] Freeing region of size {} at 0x{:X}.", size, start);

        let mut node = MemoryNode::new(size);
        node.next = self.list.next.take();
        let ptr = start as *mut MemoryNode;
        ptr.write(node);
        self.list.next = Some(&mut *ptr);
    }

    pub unsafe fn find_region(&mut self, size: usize, align: usize) -> Option<Region> {
        let mut current = &mut self.list;

        while let Some(ref mut node) = current.next {
            if let Ok((alloc_start, alloc_end)) = Self::region_from_node(size, align, node) {
                let next = node.next.take();
                let region = Region::new(current.next.take().unwrap(), alloc_start, alloc_end);
                current.next = next;
                return Some(region);
            } else {
                current = current.next.as_mut().unwrap();
            }
        }

        // TODO: merge blocks if nothing is found
        return None;
    }

    fn region_from_node(size: usize, align: usize, node: &MemoryNode) -> Result<(usize, usize), ()> {
        let alloc_start = align_address(node.start_address(), align);
        let alloc_end = align_address(alloc_start + size, core::mem::align_of::<MemoryNode>());

        if node.end_address() < alloc_end {
            return Err(())
        }

        let remainder = node.end_address() - alloc_end;
        if remainder != 0 && remainder < core::mem::size_of::<MemoryNode>() {
            return Err(());
        }

        return Ok((alloc_start, alloc_end));
    }

    fn aligned_layout(layout: Layout) -> Layout {
        layout
            .align_to(core::mem::align_of::<MemoryNode>())
            .expect("Failed to align Layout to contain MemoryNode.")
            .pad_to_align()
    }

    fn actual_size(layout: Layout) -> usize {
        layout.size().max(core::mem::size_of::<MemoryNode>())
    }
}

unsafe impl GlobalAlloc for Locked<LinkedHeap> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let aligned_layout = LinkedHeap::aligned_layout(layout);
        if let Some(region) = allocator.find_region(
            aligned_layout.size().max(core::mem::size_of::<MemoryNode>()),
            LinkedHeap::actual_size(layout),
        ) {
            let remainder = region.node.end_address() - region.alloc_end;
            if remainder > 0 {
                let aligned = align_address(region.alloc_end, core::mem::align_of::<MemoryNode>());
                allocator.free_region(aligned, region.node.end_address() - aligned);
            }
            return region.alloc_start as *mut u8;
        } else {
            return core::ptr::null_mut()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = LinkedHeap::actual_size(LinkedHeap::aligned_layout(layout));
        let mut allocator = self.lock();
        allocator.free_region(ptr as usize, size);
    }
}

#[os_test]
fn mem_allocator_create_box() {
    let boxed = Box::new(412);
    assert_eq!(*boxed.deref(), 412);
}

#[os_test]
fn mem_allocator_create_vector() {
    let mut vec = alloc::vec!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
    assert_eq!(*vec.index(3), 3);

    for i in 10..100 {
        vec.push(i)
    }
    assert_eq!(*vec.index(97), 97);
}

