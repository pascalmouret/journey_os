use crate::mem::address::VirtualAddress;
use crate::mem::frames::FrameSize;
use crate::mem::paging::mapper::map_frame;
use crate::mem::frames::FRAME_MAP;
use crate::mem::paging::table::Table;

static KERNEL_HEAP_START: usize = 0x4000_0000_0000;
static KERNEL_HEAP_SIZE: usize = 1024 * 1024;

pub unsafe fn init_heap() {
    let paging_table = Table::load_current();
    let frames = KERNEL_HEAP_SIZE / FrameSize::SMALL as usize + usize::from(KERNEL_HEAP_SIZE % FrameSize::SMALL as usize != 0);

    for i in 0..frames {
        let frame = FRAME_MAP.lock().alloc_free();
        map_frame(
            &frame,
            &VirtualAddress::new(KERNEL_HEAP_START + FrameSize::SMALL as usize * i),
            paging_table,
        );
    }

    crate::logln!("[allocator] Built 0x{:X} byte kernel heap at 0x{:X}.", KERNEL_HEAP_SIZE, KERNEL_HEAP_START);
}
