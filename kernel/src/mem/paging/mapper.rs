use macros::os_test;
use crate::mem::address::VirtualAddress;
use crate::mem::frames::{Frame, FrameSize};
use crate::mem::paging::table::{Level4, Table};

pub unsafe fn map_frame(frame: &Frame, target: &VirtualAddress, table: &mut Table<Level4>) {
    let l3 = table.get_or_create_next(target.l4_index());

    if frame.size == FrameSize::HUGE {
        l3.set(target.l3_index(), &frame.start_address, true);
    }

    let l2 = l3.get_or_create_next(target.l3_index());

    if frame.size == FrameSize::LARGE {
        l2.set(target.l2_index(), &frame.start_address, true);
    }

    l2.get_or_create_next(target.l1_index())
        // don't set is page flag on l1 pages
        .set(target.l1_index(), &frame.start_address, false);
}

#[os_test]
fn mem_paging_mapper_map_frame() {
    let table = Table::load_current();
    let frame = FRAME_MAP.lock().alloc_free();
    let target = VirtualAddress::new(0xFFFFFFFFFFF);

    unsafe {
        map_frame(&frame, &target, table);

        let ptr = target.data() as *mut u8;
        ptr.write(42);

        assert_eq!(ptr.read(), 42);
    }
}
