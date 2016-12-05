#![allow(dead_code)]

// Boot loader.
//
// Part of the boot block, along with bootasm.S, which calls bootmain().
// bootasm.S has put the processor into protected 32-bit mode.
// bootmain() loads an ELF kernel image from the disk starting at
// sector 1 and then jumps to the kernel entry routine.

use core;
use elf::*;
use x86::*;

const SECTSIZE: usize = 512;

#[no_mangle]
pub unsafe extern "C" fn bootmain() {
    let elf = 0x10000 as *mut elfhdr;  // scratch space

    // Read 1st page off disk
    readseg(elf as usize, 4096, 0);

    // Is this an ELF executable?
    if (*elf).magic != ELF_MAGIC {
        return;  // let bootasm.S handle error
    }

    // Load each program segment (ignores ph flags).
    let bph = (elf as usize) + ((*elf).phoff as usize);
    let eph = bph + ((*elf).phnum as usize);
    for iph in bph..eph {
        let ph = iph as *mut proghdr;
        let pa = (*ph).paddr;
        readseg(pa, (*ph).filesz, (*ph).off);
        if (*ph).memsz > (*ph).filesz {
            stosb(pa + (*ph).filesz, 0, (*ph).memsz - (*ph).filesz);
        }
    }

    // Call the entry point from the ELF header.
    // Does not return!
    let entry: extern "C" fn () -> ! = core::mem::transmute((*elf).entry);
    entry();
}

unsafe fn waitdisk() {
    while (inb(0x1F7) & 0xC0) != 0x40 {
    }
}

// Read a single sector at offset into dst.
unsafe fn readsect(dst: usize, offset: usize) {
    // Issue command.
    waitdisk();
    outb(0x1F2, 1 as u8);   // count = 1
    outb(0x1F3, offset as u8);
    outb(0x1F4, (offset >> 8) as u8);
    outb(0x1F5, (offset >> 16) as u8);
    outb(0x1F6, ((offset >> 24) | 0xE0) as u8);
    outb(0x1F7, 0x20);  // cmd 0x20 - read sectors

    // Read data.
    waitdisk();
    insl(0x1F0, dst, SECTSIZE / 4);
}

// Read 'count' bytes at 'offset' from kernel into physical address 'pa'.
// Might copy more than asked.
#[inline(never)]
unsafe fn readseg(pa: usize, count: usize, offset: usize) {
    let epa = pa + count;

    // Round down to sector boundary.
    let bpa = pa - offset % SECTSIZE;

    // Translate from bytes to sectors; kernel starts at sector 1.
    let mut offset = (offset / SECTSIZE) + 1;

    // If this is too slow, we could read lots of sectors at a time.
    // We'd write more to memory than asked, but it doesn't matter --
    // we load in increasing order.
    for pa in (bpa..epa).step_by(SECTSIZE) {
        readsect(pa, offset);
        offset += 1;
    }
}
