#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern "C" fn start() -> ! {
    loop { }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
