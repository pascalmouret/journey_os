use core::arch::asm;

use macros::os_test;
use crate::mem::address::VirtualAddress;
use crate::mem::frames::{Frame, FRAME_MAP, FrameSize};
use crate::mem::paging::table::{Level4, Table};

pub unsafe fn map_frame(frame: &Frame, target: &VirtualAddress, l4: &mut Table<Level4>) {
    assert_eq!(target.data() % frame.size as usize, 0);

    crate::logln!(
        "[allocator] Mapping frame 0x{:X} to 0x{:X} with root table 0x{:X}.",
        frame.start_address.data(),
        target.data(),
        l4 as *mut Table<Level4> as usize,
    );

    let l3 = l4.get_or_create_next(target.l4_index());

    if frame.size == FrameSize::HUGE {
        l3.set(target.l3_index(), &frame.start_address, true);
        return;
    }

    let l2 = l3.get_or_create_next(target.l3_index());

    if frame.size == FrameSize::LARGE {
        l2.set(target.l2_index(), &frame.start_address, true);
        return;
    }

    l2.get_or_create_next(target.l2_index())
        // don't set is page flag on l1 pages
        .set(target.l1_index(), &frame.start_address, false);

    // TODO: only flush affected pages
    asm!(
        "mov %cr3, %rax",
        "mov %rax, %cr3",
        options(att_syntax),
    )
}

#[os_test]
fn mem_paging_mapper_map_frame() {
    let table = Table::load_current();
    let frame = FRAME_MAP.lock().alloc_free();
    let target = VirtualAddress::new(0x3000_0000_0000);

    unsafe {
        map_frame(&frame, &target, table);

        let ptr = target.data() as *mut u8;
        let current = ptr.read();

        ptr.write(43);
        assert_eq!(ptr.read(), 43);

        ptr.write(current);
    }
}
