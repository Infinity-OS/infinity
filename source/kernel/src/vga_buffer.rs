//! # VGA Console Implementation

use core::ptr::Unique;
use core::fmt;
use spin::Mutex;
use volatile::Volatile;

//macros  definition
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

// Constants
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
struct ColorCode(u8);

impl ColorCode {
    const fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

pub struct Writer {
    row_positon: usize,
    column_position: usize,
    color_code: ColorCode,
    buffer: Unique<Buffer>,
}

pub static WRITER: Mutex<Writer> = Mutex::new(Writer {
    row_positon: 0,
    column_position: 0,
    color_code: ColorCode::new(Color::Cyan, Color::White),
    buffer: unsafe { Unique::new(0xffffff8000005000 as *mut _) },
});

/// Implement the Writer Struct
impl Writer {

    /// Write a byte to the console.
    ///
    /// # Arguments
    ///
    /// * `byte` - Byte to be writen
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }
                let row = self.row_positon;
                let col = self.column_position;

                let color_code = self.color_code;

                self.buffer().chars[row][col].write(ScreenChar {
                    ascii_character: byte,
                    color_code: color_code,
                });
                self.column_position += 1;
            }
        }
    }

    /// Gets a mutable reference to the console Buffer.
    fn buffer(&mut self) -> &mut Buffer {
        unsafe { self.buffer.get_mut() }
    }

    /// Adds a new file
    fn new_line(&mut self) {
        // increment the current row
        self.row_positon += 1;

        // reset the column positon
        self.column_position = 0;

        // scroll the view if there is no more free rows
        if self.row_positon == BUFFER_HEIGHT {
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let buffer = self.buffer();
                    let character = buffer.chars[row][col].read();
                    buffer.chars[row - 1][col].write(character);
                }
            }

            // clear the last row
            self.clear_row(BUFFER_HEIGHT - 1);

            // positon the cursor on the last row
            self.row_positon -= 1;
        }
    }

    /// Clear a full row
    fn clear_row(&mut self, row: usize) {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            self.buffer().chars[row][col].write(blank);
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub fn print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}

/// Clear screen-
pub fn clear_screen() {
    // iterate all rows and clear them all
    for i in 0..BUFFER_HEIGHT {
        WRITER.lock().clear_row(i);
    }

    // reset the row position
    WRITER.lock().row_positon = 0;
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte)
        }
        Ok(())
    }
}

extern "C" fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}
