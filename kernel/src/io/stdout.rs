use spin::Mutex;
use lazy_static::lazy_static;
use crate::io::vga::CONSOLE;

lazy_static! {
    pub static ref STD_OUT: Mutex<StdOut> = Mutex::new(StdOut::new(&CONSOLE));
}

pub trait StdOutWriter {
    fn write(&mut self, s: &str);
}

// TODO: allow non static writers
pub struct StdOut {
    writer: &'static Mutex<dyn StdOutWriter + Send>,
}

impl core::fmt::Write for StdOut {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.writer.lock().write(s);
        Ok(())
    }
}

impl StdOut {
    fn new<A: StdOutWriter + Send>(writer: &'static Mutex<A>) -> StdOut {
        StdOut { writer }
    }

    pub fn set<A: StdOutWriter + Send>(&mut self, writer: &'static Mutex<A>) {
        self.writer = writer;
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::stdout::_print(format_args!($($arg)*)));
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
