#![allow(dead_code)]

// Intel 8250 serial port (UART).

use defs::*;
use traps;
use x86;

const COM1: u16 = 0x3f8;

static mut UART: usize = 0; // is there a uart?

#[no_mangle]
pub unsafe fn uartinit() {
    // Turn off the FIFO
    x86::outb(COM1 + 2, 0);

    // 9600 baud, 8 data bits, 1 stop bit, parity off.
    x86::outb(COM1 + 3, 0x80);    // Unlock divisor
    x86::outb(COM1 + 0, (115200/9600) as u8);
    x86::outb(COM1 + 1, 0);
    x86::outb(COM1 + 3, 0x03);    // Lock divisor, 8 data bits.
    x86::outb(COM1 + 4, 0);
    x86::outb(COM1 + 1, 0x01);    // Enable receive interrupts.

    // If status is 0xFF, no serial port.
    if x86::inb(COM1 + 5) == 0xFF {
        return;
    }
    UART = 1;

    // Acknowledge pre-existing interrupt conditions;
    // enable interrupts.
    x86::inb(COM1 + 2);
    x86::inb(COM1 + 0);
    picenable(traps::IRQ_COM1 as i32);
    ioapicenable(traps::IRQ_COM1 as i32, 0);

    // Announce that we're here.
    for p in "xv6...\n".as_bytes() {
        uartputc(*p as isize);
    }
}

#[no_mangle]
pub unsafe fn uartputc(c: isize) {
    if UART == 0 {
        return;
    }
    for _ in 0..128 {
        if x86::inb(COM1 + 5) & 0x20 != 0 {
            break;
        }
        microdelay(10);
    }
    x86::outb(COM1 + 0, c as u8);
}

#[no_mangle]
pub unsafe extern "C" fn uartgetc() -> isize {
    if UART == 0 {
        return -1;
    }
    if x86::inb(COM1 + 5) & 0x01 == 0 {
        return -1;
    }
    return x86::inb(COM1 + 0) as isize;
}

#[no_mangle]
pub unsafe fn uartintr() {
    consoleintr(uartgetc);
}
