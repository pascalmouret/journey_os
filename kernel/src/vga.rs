use core;
use spin::Mutex;
use lazy_static::lazy_static;

const VGA_BUFFER_ADDR: usize = 0xb8000;
const COLUMNS: usize = 80;
const ROWS: usize = 25;

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console::new());
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}

#[allow(dead_code)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[allow(dead_code)] // we never read the color, only store it in the buffer
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ColorCode {
    color: u8
}

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode { color: foreground as u8 + ((background as u8) << 4) }
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct VgaChar {
    character: u8,
    color: ColorCode,
}

struct Cursor {
    row: usize,
    column: usize,
}

#[repr(transparent)]
struct Buffer {
    chars: [[VgaChar; COLUMNS]; ROWS],
}

pub struct Console {
    current_color: ColorCode,
    cursor: Cursor,
    buffer: &'static mut Buffer,
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write(s);
        Ok(())
    }
}

impl Console {
    fn new() -> Console {
        let mut console = Console {
            current_color: ColorCode::new(Color::White, Color::Black),
            cursor: Cursor { row: 0, column: 0 },
            buffer: unsafe { &mut *(VGA_BUFFER_ADDR as *mut Buffer) },
        };
        console.clear();
        console
    }

    pub fn write(&mut self, string: &str) {
        for char in string.bytes() {
            self.write_byte(char);
        }
    }

    pub fn clear(&mut self) {
        for r in 0..ROWS {
            for c in 0..COLUMNS {
                self.buffer.chars[r][c] = VgaChar { character: 0, color: self.current_color }
            }
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if self.cursor.row == ROWS {
            self.scroll();
        }

        match byte {
            b'\n' => {
                self.cursor.row += 1;
                self.cursor.column = 0;
            }
            byte => {
                let color = self.current_color;
                self.buffer.chars[self.cursor.row][self.cursor.column] = VgaChar { character: byte, color };
                self.cursor.column += 1;
                if self.cursor.column == COLUMNS {
                    self.cursor.column = 0;
                    self.cursor.row += 1;
                }
            }
        }
    }

    fn scroll(&mut self) {
        for r in 1..ROWS {
            self.buffer.chars[r - 1] = self.buffer.chars[r];
        }
        for c in 0..COLUMNS {
            self.buffer.chars[ROWS - 1][c] = VgaChar { character: 0, color: self.current_color }
        }
        self.cursor.row -= 1;
    }
}
