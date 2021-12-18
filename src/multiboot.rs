/*
The format of the Multiboot information structure (as defined so far) follows:

        +-------------------+
0       | flags             |    (required)
        +-------------------+
4       | mem_lower         |    (present if flags[0] is set)
8       | mem_upper         |    (present if flags[0] is set)
        +-------------------+
12      | boot_device       |    (present if flags[1] is set)
        +-------------------+
16      | cmdline           |    (present if flags[2] is set)
        +-------------------+
20      | mods_count        |    (present if flags[3] is set)
24      | mods_addr         |    (present if flags[3] is set)
        +-------------------+
28 - 40 | syms              |    (present if flags[4] or
        |                   |                flags[5] is set)
        +-------------------+
44      | mmap_length       |    (present if flags[6] is set)
48      | mmap_addr         |    (present if flags[6] is set)
        +-------------------+
52      | drives_length     |    (present if flags[7] is set)
56      | drives_addr       |    (present if flags[7] is set)
        +-------------------+
60      | config_table      |    (present if flags[8] is set)
        +-------------------+
64      | boot_loader_name  |    (present if flags[9] is set)
        +-------------------+
68      | apm_table         |    (present if flags[10] is set)
        +-------------------+
72      | vbe_control_info  |    (present if flags[11] is set)
76      | vbe_mode_info     |
80      | vbe_mode          |
82      | vbe_interface_seg |
84      | vbe_interface_off |
86      | vbe_interface_len |
        +-------------------+
88      | framebuffer_addr  |    (present if flags[12] is set)
96      | framebuffer_pitch |
100     | framebuffer_width |
104     | framebuffer_height|
108     | framebuffer_bpp   |
109     | framebuffer_type  |
110-115 | color_info        |
        +-------------------+
*/

#[repr(C, packed)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u8; 16],
    pub mmap_size: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub color_info: [u8; 6],
}

#[repr(C, packed)]
pub struct MemoryMapEntry {
    pub size: u32,
    pub base: u64,
    pub limit: u64,
    pub kind: MemoryKind,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u32)]
pub enum MemoryKind {
    Unknown = 0,
    Usable = 1,
    Undefined = 2,
    ACPI = 3,
    Reserved = 4,
    Damaged = 5,
}

pub struct MemoryMapPointer {
    buffer_end: usize,
    pub entry: &'static MemoryMapEntry,
}

impl MultibootInfo {
    pub unsafe fn memory_map(&self) -> MemoryMapPointer {
        // TODO: check mmap flag
        MemoryMapPointer {
            buffer_end: (self.mmap_addr + self.mmap_size) as usize,
            entry: &*(self.mmap_addr as *const MemoryMapEntry),
        }
    }
}

impl MemoryMapPointer {
    pub fn next(&self) -> Option<MemoryMapPointer> {
        // we have to add an additional four bytes to account for the size field
        let next = (self.entry as *const MemoryMapEntry as usize) + self.entry.size as usize + 4;

        if self.buffer_end > next {
            Some(
                MemoryMapPointer {
                    buffer_end: self.buffer_end,
                    entry: unsafe { &*(next as *const MemoryMapEntry) }
                }
            )
        } else {
            None
        }
    }
}
