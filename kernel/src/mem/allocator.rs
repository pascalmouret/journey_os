use alloc::boxed::Box;
use core::alloc::{GlobalAlloc, Layout};
use core::borrow::BorrowMut;
use core::ops::{Deref, DerefMut, Index};
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

const MEMORY_NODE_SIZE: usize = core::mem::size_of::<MemoryNode>();
const MEMORY_NODE_ALIGN: usize = core::mem::align_of::<MemoryNode>();

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
    limit: usize,
}

impl LinkedHeap {
    pub const fn new() -> LinkedHeap {
        LinkedHeap { list: MemoryNode::new(0), limit: 0 }
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

        self.limit = start + size;
        self.free_region(start, size);

        crate::logln!("[allocator] Built 0x{:X} byte kernel heap at 0x{:X}.", size, start);
    }

    pub unsafe fn free_region(&mut self, start: usize, size: usize) {
        assert!(size >= MEMORY_NODE_SIZE, "Freed region too small for memory node.");
        assert_eq!(start % MEMORY_NODE_ALIGN, 0, "Freed region is not properly aligned");
        assert!(start + size <= self.limit, "Freed region exceeds heap limit.");

        let mut node = MemoryNode::new(size);
        node.next = self.list.next.take();
        let ptr = start as *mut MemoryNode;
        ptr.write(node);
        self.list.next = Some(&mut *ptr);
    }

    pub unsafe fn find_region(&mut self, size: usize, align: usize, merged: bool) -> Option<Region> {
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

        if !merged {
            self.merge_list();
            return self.find_region(size, align, true);
        }

        // TODO: expand heap if nothing is found
        return None;
    }

    fn region_from_node(size: usize, align: usize, node: &MemoryNode) -> Result<(usize, usize), ()> {
        let alloc_start = align_address(node.start_address(), align);
        let alloc_end = align_address(alloc_start + size, MEMORY_NODE_ALIGN);

        if node.end_address() < alloc_end {
            return Err(())
        }

        let remainder = node.end_address() - alloc_end;
        if remainder != 0 && remainder < MEMORY_NODE_SIZE {
            return Err(());
        }

        return Ok((alloc_start, alloc_end));
    }

    fn aligned_layout(layout: Layout) -> Layout {
        layout
            .align_to(MEMORY_NODE_ALIGN)
            .expect("Failed to align Layout to contain MemoryNode.")
            .pad_to_align()
    }

    fn actual_size(layout: Layout) -> usize {
        layout.size().max(MEMORY_NODE_SIZE)
    }

    unsafe fn merge_list(&mut self) {
        crate::logln!("[allocator] Merging allocator list.");
        // TODO: fix this cheat
        let head = &mut *(self.list.start_address() as *mut MemoryNode);
        self.merge_list_rec(head);
    }

    unsafe fn merge_list_rec(&mut self, current: &mut MemoryNode) {
        if current.next.is_none() {
            return
        }

        let adjacent = self.find_adjacent_node_parent(current.next.as_ref().unwrap());

        if let Some(adjacent_parent) = adjacent {
            let merged = Self::merge_nodes(current, adjacent_parent);
            self.free_region(merged.0, merged.1);
            self.merge_list();
        } else {
            if let Some(node) = &mut current.next {
                self.merge_list_rec(node)
            }
        }
    }

    fn find_adjacent_node_parent(&mut self, node: &MemoryNode) -> Option<&mut MemoryNode> {
        let mut current = &mut self.list;

        while let Some(candidate) = &current.next {
            if candidate.start_address() >= node.end_address()
                && candidate.start_address() - node.end_address() < MEMORY_NODE_SIZE
            {
                return Some(current)
            }

            if candidate.end_address() <= node.start_address()
                && node.start_address() - candidate.end_address() < MEMORY_NODE_SIZE
            {
                return Some(current)
            }

            current = current.next.as_mut().unwrap();
        }

        return None;
    }

    fn merge_nodes(
        node1_parent: &mut MemoryNode,
        node2_parent: &mut MemoryNode,
    ) -> (usize, usize) {
        let node1 = node1_parent.next.take().unwrap();
        let node2 = node2_parent.next.take().unwrap();

        node1_parent.next = node1.next.take();
        node2_parent.next = node2.next.take();

        let start = node1.start_address().min(node2.start_address());
        let end = node1.end_address().max(node2.end_address());

        (start, end - start)
    }
}

unsafe impl GlobalAlloc for Locked<LinkedHeap> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let aligned_layout = LinkedHeap::aligned_layout(layout);
        if let Some(region) = allocator.find_region(
            aligned_layout.size().max(core::mem::size_of::<MemoryNode>()),
            LinkedHeap::actual_size(layout),
            false,
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
fn mem_allocator_create_deallocate_box() {
    let address: usize;
    {
        let boxed = Box::new(731);
        address = boxed.deref() as *const i32 as usize;
    }

    let boxed2 = Box::new(137);
    assert_eq!(boxed2.deref() as *const i32 as usize, address);
}

#[os_test]
fn mem_allocator_create_vector() {
    {
        let mut vec = alloc::vec!();
        for i in 0..500 {
            vec.push(i)
        }
        assert_eq!(*vec.index(97), 97);
    }


    let mut boxed = Box::new(412);
    assert_eq!(*boxed.deref(), 412);
}
