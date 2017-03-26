use memory::MemoryController;

pub mod local_apic;
pub mod rtc;
pub mod serial;

/// Initialize some devices
pub fn init(memory_controller: &mut MemoryController) {
    unsafe {
        local_apic::init(memory_controller);
    }
}

/// Initialize all non core devices
pub fn init_non_core() {
    rtc::init();
    serial::init();
}
