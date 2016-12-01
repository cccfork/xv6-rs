#![feature(lang_items)]
#![no_std]

extern crate rlibc;

pub mod defs;
pub mod memlayout;
pub mod main_;

#[cfg(not(test))]
#[lang = "eh_personality"]
extern "C" fn eh_personality() {
}

#[cfg(not(test))]
#[lang = "panic_fmt"]
extern "C" fn panic_fmt() -> ! {
    loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}
