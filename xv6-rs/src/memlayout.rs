// Memory layout

#![allow(non_snake_case)]

use defs::*;

pub const EXTMEM: usize = 0x100000;     // Start of extended memory
pub const PHYSTOP: usize = 0xE000000;   // Top physical memory
pub const DEVSPACE: usize = 0xFE000000; // Other devices are at high addresses

// Key addresses for address space layout (see kmap in vm.c for layout)
pub const KERNBASE: usize = 0x80000000;          // First kernel virtual address
pub const KERNLINK: usize = (KERNBASE + EXTMEM); // Address where kernel is linked

#[inline(always)]
pub fn V2P(a: *mut void) -> usize {
    return (a as usize) - KERNBASE;
}

#[inline(always)]
pub fn P2V(a: usize) -> *mut void {
    return (a + KERNBASE) as *mut void;
}

// same as V2P, but without casts
#[inline(always)]
pub fn V2P_WO(a: usize) -> usize {
    return a - KERNBASE;
}

// same as P2V, but without casts
#[inline(always)]
pub fn P2V_WO(a: usize) -> usize {
    return a + KERNBASE;
}
