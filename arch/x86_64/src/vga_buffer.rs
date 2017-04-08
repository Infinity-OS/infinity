//! # VGA Console Implementation

use core::ptr::Unique;
use core::fmt;
use spin::Mutex;
use volatile::Volatile;

use device::serial::COM1;

/// Print with new line to console
#[macro_export]
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

/// Print to console
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
            $crate::vga_buffer::print(format_args!($($arg)*));
    });
}

// VGA screen dimentions
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

const DEFAULT_COLOR_CODE: ColorCode = ColorCode::new(Color::Cyan, Color::White);

#[derive(Debug, Clone, Copy)]
pub struct ColorCode(u8);

impl ColorCode {
    pub const fn new(foreground: Color, background: Color) -> ColorCode {
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
    color_code: DEFAULT_COLOR_CODE,
    buffer: unsafe { Unique::new(0xb8000 as *mut _) },
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
            b'\t' => for i in 0..2 { self.write_byte(b' ') },
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
    /// Change the colors
    pub fn set_colors(&mut self, foreground: Color, background: Color) {
        self.color_code = ColorCode::new(foreground, background)
    }

    /// Change the color code
    pub fn set_color_code(&mut self, color_code: ColorCode) {
        self.color_code = color_code;
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

        // If the console is enable, also print out to the console
        if COM1.lock().is_enable() {
            COM1.lock().write_str(s);
        }

        Ok(())
    }
}

extern "C" fn panic_fmt(_: ::core::fmt::Arguments, _: &'static str, _: u32) -> ! {
    loop {}
}
