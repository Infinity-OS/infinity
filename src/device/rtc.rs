//! RTC Manager Module
//!
//! References:
//! - http://wiki.osdev.org/RTC
//! - http://www.ousob.com/ng/interrupts_and_ports/ng918f7.php

use x86_64::instructions::port::{outb, inb};
use time;

/// Initialize RTC
pub fn init() {
    let mut rtc = Rtc::new();
    let cur_time = rtc.time();
    time::START.lock().0 = cur_time;

    println!("RTC: Initialized\nCurrent epoch: {:}", cur_time);
}

/// Convert BSD to binary
fn cvt_bcd(value: usize) -> usize {
    (value & 0xF) + ((value / 16) * 10)
}

/// RTC
pub struct Rtc {
    address: u16,
    data: u16
}

impl Rtc {
    /// Create a new empty RTC
    pub fn new() -> Self {
        return Rtc {
            address: 0x70,
            data: 0x71
        }
    }

    /// Read
    fn read(&mut self, reg: u8) -> u8 {
        unsafe {
            // select the registers from where we will read
            outb(self.address, reg);

            // read the data
            inb(self.data)
        }
    }

    /// Wait
    pub fn wait(&mut self) {
        while self.read(0xA) & 0x80 != 0x80 {}
        while self.read(0xA) & 0x80 != 0x80 {}
    }

    /// Get current time in epoch format.
    pub fn time(&mut self) -> u64 {
        use bit_field::BitField;

        let mut second;
        let mut minute;
        let mut hour;
        let mut day;
        let mut month;
        let mut year;
        let mut century;
        let mut register_b;

        unsafe {
            self.wait();
            second = self.read(0) as usize;
            minute = self.read(2) as usize;
            hour = self.read(4) as usize;
            day = self.read(7) as usize;
            month = self.read(8) as usize;
            year = self.read(9) as usize;
            // TODO use ACPI FADT to get the current century. VirtualBox introduces some challenges.
            century = 20 as usize;
            register_b = self.read(0xb);

            // If the bit 4 is set, that means that the system used BCD instead of binary numbers
            if !register_b.get_bit(4) {
                second = cvt_bcd(second);
                minute = cvt_bcd(minute);
                hour = cvt_bcd(hour);
                day = cvt_bcd(day);
                month = cvt_bcd(month);
                year = cvt_bcd(year);
                // TODO implement century correctly
                // century = cvt_bcd(century);
            }

            // If the bit 2 is set that means that the system is using the a 24-hour mode.
            if register_b.get_bit(2) || hour & 0x80 == 0x80 {
                hour = ((hour & 0x7f) + 12) % 24;
            }

            // compute the current year
            year += century * 100;

            // Unix time from clock
            let mut secs: u64 = (year as u64 - 1970) * 31536000;

            // check if the year is a leap year.
            let mut leap_days = (year as u64 - 1972) / 4 + 1;
            if year % 4 == 0 {
                if month <= 2 {
                    leap_days -= 1;
                }
            }
            secs += leap_days * 86400;

            // add the seconds of the passed months
            match month {
                2 => secs += 2678400,
                3 => secs += 5097600,
                4 => secs += 7776000,
                5 => secs += 10368000,
                6 => secs += 13046400,
                7 => secs += 15638400,
                8 => secs += 18316800,
                9 => secs += 20995200,
                10 => secs += 23587200,
                11 => secs += 26265600,
                12 => secs += 28857600,
                _ => (),
            }

            secs += (day as u64 - 1) * 86400;
            secs += hour as u64 * 3600;
            secs += minute as u64 * 60;
            secs += second as u64;

            secs
        }
    }
}

// epoch time
