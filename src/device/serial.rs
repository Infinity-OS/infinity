//! Serial Device Manager
//!
//! References:
//! - http://retired.beyondlogic.org/serial/serial.htm

use x86_64::instructions::port::{outb, inb};

use core::fmt::{self, Write};
use spin::Mutex;

pub static COM1: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x3f8));
pub static COM2: Mutex<SerialPort> = Mutex::new(SerialPort::new(0x2f8));

/// Initialize the two serials.
pub fn init() {
    COM1.lock().init();
    COM2.lock().init();

    println!("Serial: Initialized");
}

bitflags! {
    /// Interrupt enable flags
    flags IntEnFlags: u8 {
        const RECEIVED = 1,
        const SENT = 1 << 1,
        const ERRORED = 1 << 2,
        const STATUS_CHANGE = 1 << 3,
        // 4 to 7 are unused
    }
}

bitflags! {
    /// Line status flags
    flags LineStsFlags: u8 {
        const INPUT_FULL = 1,
        // 1 to 4 unknown
        const OUTPUT_EMPTY = 1 << 5,
        // 6 and 7 unknown
    }
}

/// Structure for a Serial Port
pub struct SerialPort {
    /// Data register, read to receive, write to send
    data: u16,
    /// Interrupt enable
    int_en: u16,
    /// FIFO control
    fifo_ctrl: u16,
    /// Line control
    line_ctrl: u16,
    /// Modem control
    modem_ctrl: u16,
    /// Line status (read-only)
    line_sts: u16,
    /// Modem status (read-only)
    modem_sts: u16,
    // This inform if the console is enabled
    enabled: bool
}

impl SerialPort {
    /// Create a new SerialPort instance.
    ///
    /// ## Parameters
    /// - `base` - base address for the serial port.
    ///
    /// ## Returns
    /// A new SerialPort instance.
    const fn new(base: u16) -> Self {
        SerialPort {
            data: base,
            int_en: base + 1,
            fifo_ctrl: base + 2,
            line_ctrl: base + 3,
            modem_ctrl: base + 4,
            line_sts: base + 5,
            modem_sts: base + 6,
            enabled: false
        }
    }

    /// Initialize the serial port.
    pub fn init(&mut self) {
        unsafe {
            use bit_field::BitField;

            // Disable all interrupts
            outb(self.int_en, 0x00);
            // Enable DLAB (set baud rate divisor)
            outb(self.line_ctrl, 0x80);
            // Set divisor to 3 (lo byte) 38400 baud
            outb(self.data, 0x03);
            // Set divisor to 3 (hi byte) 38400 baud
            outb(self.int_en, 0x00);
            // 8 bits, no parity, one stop bit
            outb(self.line_ctrl, 0x03);
            // Enable FIFO, clear them, with 14-byte threshold
            outb(self.fifo_ctrl, 0xc7);
            // IRQs enabled, RTS/DSR set
            outb(self.modem_ctrl, 0x0b);

            // Wait for transmit to be empty
            while ! inb(self.line_sts).get_bit(5) {}

            // set the console as enabled
            self.enabled = true
        }
    }

    /// True is the console is enable.
    ///
    /// To a console be enable the init functions must be called first.
    pub fn is_enable(&self) -> bool {
        self.enabled
    }

    fn line_sts(&self) -> LineStsFlags {
        LineStsFlags::from_bits_truncate(unsafe { inb(self.line_sts) })
    }

    /// Write one byte for the serial port.
    ///
    /// ## Parameters
    /// - `data` - Byte to be writen.
    fn write(&mut self, data: u8) {
        while ! self.line_sts().contains(OUTPUT_EMPTY) {}
        unsafe { outb(self.data, data); }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        for byte in s.bytes() {
            match byte {
                8 | 0x7f => {
                    self.write(8);
                    self.write(b' ');
                    self.write(8);
                },
                _ => {
                    self.write(byte);
                }
            }
        }

        Ok(())
    }
}
