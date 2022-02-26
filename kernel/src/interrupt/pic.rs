use spin::Mutex;
use lazy_static::lazy_static;

use crate::io::port::Port;

const PRIMARY_DATA: u16 = 0x20;
const PRIMARY_COMMAND: u16 = 0x21;
const SECONDARY_DATA: u16 = 0x20;
const SECONDARY_COMMAND: u16 = 0x21;

lazy_static! {
    pub static ref PIC_PRIMARY: Mutex<PIC> = unsafe {
        Mutex::new(PIC::new(PRIMARY_DATA, PRIMARY_COMMAND))
    };
    pub static ref PIC_SECONDARY: Mutex<PIC> = unsafe {
        Mutex::new(PIC::new(SECONDARY_DATA, SECONDARY_COMMAND))
    };
}

pub struct PIC {
    command: Port<u8>,
    data: Port<u8>,
}

impl PIC {
    pub unsafe fn new(command_port: u16, data_port: u16) -> PIC {
        return PIC {
            command: Port::<u8>::open(command_port),
            data: Port::<u8>::open(data_port),
        }
    }

    pub fn disable(&self) {
        self.command.write(0xFF);
    }
}
