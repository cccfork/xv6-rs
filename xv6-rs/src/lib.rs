#![feature(asm)]
#![feature(lang_items)]
#![no_std]

extern crate rlibc;

pub mod defs;
#[macro_use]
pub mod memlayout;
pub mod x86;
pub mod console;
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
