#![feature(asm)]
#![feature(lang_items)]
#![feature(step_by)]
#![no_std]

extern crate rlibc;

pub mod defs;
pub mod param;
#[macro_use]
pub mod memlayout;
pub mod x86;
pub mod mmu;
pub mod elf;
pub mod bootmain;
pub mod traps;
pub mod uart;
pub mod console;
pub mod main_;
pub mod log;
pub mod fs;
pub mod exec;

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
