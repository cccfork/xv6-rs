// Memory layout

#![allow(non_snake_case,
         unused_imports)]

use defs::*;

pub const EXTMEM: usize = 0x100000;     // Start of extended memory
pub const PHYSTOP: usize = 0xE000000;   // Top physical memory
pub const DEVSPACE: usize = 0xFE000000; // Other devices are at high addresses

// Key addresses for address space layout (see kmap in vm.c for layout)
pub const KERNBASE: usize = 0x80000000;          // First kernel virtual address
pub const KERNLINK: usize = (KERNBASE + EXTMEM); // Address where kernel is linked

macro_rules! V2P {
    ($e:expr) => (($e as usize) - KERNBASE);
}

macro_rules! P2V {
    ($e:expr) => (($e + KERNBASE) as *mut void);
}

// same as V2P, but without casts
macro_rules! V2P_WO {
    ($e:expr) => ($e - KERNBASE);
}

// same as P2V, but without casts
macro_rules! P2V_WO {
    ($e:expr) => ($e + KERNBASE);
}
