use spin::Mutex;
use lazy_static::lazy_static;
use crate::io::vga::CONSOLE;
use crate::io::serial::COM1;

lazy_static! {
    pub static ref STD_OUT: Mutex<Output> = Mutex::new(Output::new(&CONSOLE));
    pub static ref LOG_OUT: Mutex<Output> = Mutex::new(Output::new(&COM1));
}

pub trait StdOutWriter {
    fn write(&mut self, s: &str);
}

// TODO: allow non static writers
pub struct Output {
    writer: &'static Mutex<dyn StdOutWriter + Send>,
}

impl core::fmt::Write for Output {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.writer.lock().write(s);
        Ok(())
    }
}

impl Output {
    fn new<A: StdOutWriter + Send>(writer: &'static Mutex<A>) -> Output {
        Output { writer }
    }

    pub fn set<A: StdOutWriter + Send>(&mut self, writer: &'static Mutex<A>) {
        self.writer = writer;
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::output::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
     use core::fmt::Write;
     STD_OUT.lock().write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => ($crate::io::output::_log(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! logln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::log!("[kernel] {}\n", format_args!($($arg)*)));
}

pub fn _log(args: core::fmt::Arguments) {
    use core::fmt::Write;
    LOG_OUT.lock().write_fmt(args).unwrap();
}
