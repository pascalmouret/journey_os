use crate::io::port::Port;
use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref COM1: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::open(0x3F8) });
    pub static ref COM2: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::open(0x2F8) });
    pub static ref COM3: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::open(0x3E8) });
    pub static ref COM4: Mutex<SerialPort> = Mutex::new(unsafe { SerialPort::open(0x2E8) });
}

pub struct SerialPort {
    data: Port,
    interrupt: Port,
    int_ident_fifo: Port,
    line_control: Port,
    modem_control: Port,
    line_status: Port,
    // modem_status: Port,
    // scratch: Port,
}

impl crate::io::stdout::StdOutWriter for SerialPort {
    fn write(&mut self, s: &str) {
        self.write_str(s);
    }
}

impl SerialPort {
    // TODO: setup proper bitmaps, for now I just need it to print
    pub unsafe fn open(port: u16) -> SerialPort {
        let serial = SerialPort {
            data: Port::open(port),
            interrupt: Port::open(port + 1),
            int_ident_fifo: Port::open(port + 2),
            line_control: Port::open(port + 3),
            modem_control: Port::open(port + 4),
            line_status: Port::open(port + 5),
            // modem_status: Port::open(port + 6),
            // scratch: Port::open(port + 7),
        };

        serial.interrupt.write(0x00 as u8);       // Disable interrupts
        serial.line_control.write(0x80 as u8);    // Enable DLAB to set baud rate divisor
        serial.data.write(0x03 as u8);            // Baud divisor 3 (38400 baud)
        serial.interrupt.write(0x00 as u8);       // Empty high byte for baud divisor
        serial.line_control.write(0x03 as u8);    // 8bit characters, 1bit stop, no parity
        serial.int_ident_fifo.write(0xC7 as u8);  // Enable FIFO, clear with 14 byte threshold
        serial.modem_control.write(0x0B as u8);   // IRQs enabled, RTS/DSR set

        // test port in loopback mode
        serial.modem_control.write(0x1E as u8);   // Set to loopback mode
        serial.data.write(0x42 as u8);
        let result: u8 = serial.data.read();
        if result != 0x42 {
            panic!("Expected loopback to return 0x42, received {:X}.", result);
        }
        serial.modem_control.write(0x0F as u8);   // back to normal operation

        return serial;
    }

    pub fn write(&self, byte: u8) {
        while self.line_status.read::<u8>() & 0x20 == 0 {}    // wait for empty buffer
        self.data.write(byte);
    }

    pub fn write_str(&self, str: &str) {
        for byte in str.bytes() {
            self.write(byte);
        }
    }
}
