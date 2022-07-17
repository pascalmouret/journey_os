use spin::Mutex;
use lazy_static::lazy_static;

use crate::io::port::Port;

// PICs
const PRIMARY_COMMAND: u16 = 0x20;
const PRIMARY_DATA: u16 = 0x21;
const SECONDARY_COMMAND: u16 = 0xA0;
const SECONDARY_DATA: u16 = 0xA1;

#[repr(u8)]
enum PicCommand {
    EndOfInterrupt = 0x20,
    Disable = 0xFF,
    // ICW4 + Init, 2 Pics, leave defaults
    Icw1 = 0b00010001,
    // x86 mode
    Icw4 = 0b00000001,
}

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
        let pic = PIC {
            command: Port::<u8>::open(command_port),
            data: Port::<u8>::open(data_port),
        };

        pic.disable();

        return pic;
    }

    pub unsafe fn initialize(primary_base: u8, secondary_base: u8) {
        let primary = PIC_PRIMARY.lock();
        let secondary = PIC_SECONDARY.lock();

        // mask all interrupts
        primary.send_data(0b11111111);
        secondary.send_data(0b11111111);

        primary.send_command(PicCommand::Icw1);
        secondary.send_command(PicCommand::Icw1);

        primary.send_data(primary_base);
        secondary.send_data(secondary_base);

        // primary on line 2
        primary.send_data(0x4);
        // secondary on line 2
        secondary.send_data(0x2);

        primary.send_command(PicCommand::Icw4);
        secondary.send_command(PicCommand::Icw4);

        primary.send_data(0b00000000);
        secondary.send_data(0b00000000);
    }

    pub fn end_of_interrupt(irq: u8) {
        if irq >= 8 {
            PIC_SECONDARY.lock().send_command(PicCommand::EndOfInterrupt);
        }
        PIC_PRIMARY.lock().send_command(PicCommand::EndOfInterrupt);
    }

    pub fn disable(&self) {
        self.send_command(PicCommand::Disable);
    }

    fn send_command(&self, command: PicCommand) {
        self.command.write(command as u8)
    }

    fn send_data(&self, data: u8) {
        self.data.write(data);
    }
}
