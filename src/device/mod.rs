use memory::MemoryController;

pub mod local_apic;

/// Initialize some devices
pub fn init(memory_controller: &mut MemoryController) {
    unsafe {
        local_apic::init(memory_controller);
    }
}
