#![allow(dead_code)]

use defs::*;
use memlayout::*;
use x86::*;

// Console input and output.
// Input is from the keyboard or serial port.
// Output is written to the screen and serial port.

extern "C" {
    pub fn uartputc(c: isize);
    pub fn cgaputc(c: isize);

    pub static panicked: isize;
}

#[no_mangle]
pub extern "C" fn printint(x: i32, base: i32, sign: i32) {
    static DIGITS: &'static str = "0123456789abcdef";

    let mut buf: [u8; 16] = [0; 16];

    let sign = if sign == 0 { false } else { x < 0 };
    let mut x = if sign { -x } else { x };
    let mut i: usize = 0;
    loop {
        buf[i] = DIGITS.as_bytes()[(x % base) as usize];
        i += 1;
        x /= base;
        if x == 0 {
            break
        }
    }

    if sign {
        buf[i] = '-' as u8;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        consputc(buf[i] as isize);
    }
}

const BACKSPACE: isize = 0x100;
const CRTPORT: u16 = 0x3d4;
const CRT: *mut u16 = P2V!(0xb8000) as *mut u16;  // CGA memory

#[no_mangle]
pub extern "C" fn consputc(c: isize) {
    unsafe {
        if panicked != 0 {
            cli();
            loop {}
        }

        if c == BACKSPACE {
            uartputc('\x08' as isize);
            uartputc(' ' as isize);
            uartputc('\x08' as isize);
        } else {
            uartputc(c);
        }
        cgaputc(c);
    }
}
