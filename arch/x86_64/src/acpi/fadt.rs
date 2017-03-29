//! Fixed ACPI Description Table (FADT)
//!
//! The Fixed ACPI Description Table (FADT) defines various fixed hardware ACPI information vital to
//! an ACPI-compatible OS, such as the base address for the following hardware registers blocks:
//! PM1a_EVT_BLK, PM1b_EVT_BLK, PM1a_CNT_BLK, PM1b_CNT_BLK, PM2_CNT_BLK, PM_TMR_BLK, GPE0_BLK, and
//! GPE1_BLK.
//! The FADT also has a pointer to the DSDT that contains the Differentiated Definition Block, which
//! in turn provides variable information to an ACPI-compatible OS concerning the base system
//! design.
//! All fields in the FADT that provide hardware addresses provide processor-relative physical
//! addresses.

use core::{mem, ptr};

use super::sdt::Sdt;

#[repr(packed)]
#[derive(Debug)]
pub struct Fadt {
    pub header: Sdt,
    pub firmware_ctrl: u32,
    pub dsdt: u32,

    // field used in ACPI 1.0; no longer in use, for compatibility only
    reserved: u8,

    pub preferred_power_managament: u8,
    pub sci_interrupt: u16,
    pub smi_command_port: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub s4_bios_req: u8,
    pub pstate_control: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pm_timer_block: u32,
    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm1_control_length: u8,
    pub pm2_control_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_ength: u8,
    pub gpe1_length: u8,
    pub gpe1_base: u8,
    pub c_state_control: u8,
    pub worst_c2_latency: u16,
    pub worst_c3_latency: u16,
    pub flush_size: u16,
    pub flush_stride: u16,
    pub duty_offset: u8,
    pub duty_width: u8,
    pub day_alarm: u8,
    pub month_alarm: u8,
    pub century: u8,

    // reserved in ACPI 1.0; used since ACPI 2.0+
    pub boot_architecture_flags: u16,

    reserved2: u8,
    pub flags: u32,
}

impl Fadt {
    /// Cast the SDT to a FADT instance
    pub fn new(sdt: &'static Sdt) -> Option<Self> {
        if &sdt.signature == b"FACP" && sdt.length as usize >= mem::size_of::<Self>() {
            Some(unsafe { ptr::read((sdt as *const Sdt) as *const Self) })
        } else {
            None
        }
    }
}
