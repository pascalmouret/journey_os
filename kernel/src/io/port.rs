pub trait PortIO {
    unsafe fn write(address: u16, value: Self);
    unsafe fn read(address: u16) -> Self;
}

impl PortIO for u8 {
    unsafe fn write(address: u16, value: Self) {
        asm! {
            "OUT %al, %dx",
            in("dx") address,
            in("al") value,
            options(att_syntax),
        }
    }

    unsafe fn read(address: u16) -> Self {
        let mut value: Self;
        asm! {
            "IN %dx, %al",
            in("dx") address,
            out("al") value,
            options(att_syntax),
        }
        return value;
    }
}
impl PortIO for u16 {
    unsafe fn write(address: u16, value: Self) {
        asm! {
            "OUT %ax, %dx",
            in("dx") address,
            in("ax") value,
            options(att_syntax),
        }
    }

    unsafe fn read(address: u16) -> Self {
        let mut value: Self;
        asm! {
            "IN %dx, %ax",
            in("dx") address,
            out("ax") value,
            options(att_syntax),
        }
        return value;
    }
}
impl PortIO for u32 {
    unsafe fn write(address: u16, value: Self) {
        asm! {
            "OUT %eax, %dx",
            in("dx") address,
            in("eax") value,
            options(att_syntax),
        }
    }

    unsafe fn read(address: u16) -> Self {
        let mut value: Self;
        asm! {
            "IN %dx, %eax",
            in("dx") address,
            out("eax") value,
            options(att_syntax),
        }
        return value;
    }
}

pub struct Port {
    address: u16,
}

// TODO: make port fixed size
impl Port {
    pub unsafe fn open(address: u16) -> Port {
        Port { address }
    }

    pub fn write<A: PortIO>(&self, data: A) {
        unsafe {
            A::write(self.address, data);
        }
    }

    pub fn read<A: PortIO>(&self) -> A {
        unsafe {
            A::read(self.address)
        }
    }
}
