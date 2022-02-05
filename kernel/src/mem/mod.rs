pub(crate) mod frames;
pub(crate) mod paging;
pub(crate) mod address;
pub(crate) mod allocator;

pub(crate) static KiB: usize = 1024;
pub(crate) static MiB: usize = KiB * KiB;
pub(crate) static GiB: usize = KiB * KiB * KiB;

pub(crate) fn align_address(address: usize, align: usize) -> usize {
    let offset = address % align;
    if offset == 0 {
        address
    } else {
        address + (align - offset)
    }
}
