#![allow(improper_ctypes)]

use defs::*;
use memlayout::*;

extern "C" {
    pub fn startothers();
    pub fn mpmain() -> !;
    pub static mut kpgdir: *mut pde_t;
    pub static end: char;
}

// Bootstrap processor starts running C code here.
// Allocate a real stack and switch to it, first
// doing some setup required for memory allocator to work.
#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        kinit1(&end as *const char as *mut void, P2V(4 * 1024 * 1024)); // phys page allocator
        kvmalloc();      // kernel page table
        mpinit();        // detect other processors
        lapicinit();     // interrupt controller
        seginit();       // segment descriptors
        cprintf(b"\ncpu%d: starting xv6\n\n".as_ptr() as *const char, cpunum());
        picinit();       // another interrupt controller
        ioapicinit();    // another interrupt controller
        consoleinit();   // console hardware
        uartinit();      // serial port
        pinit();         // process table
        tvinit();        // trap vectors
        binit();         // buffer cache
        fileinit();      // file table
        ideinit();       // disk
        if ismp == 0 {
            timerinit();   // uniprocessor timer
        }
        startothers();   // start other processors
        kinit2(P2V(4*1024*1024), P2V(PHYSTOP)); // must come after startothers()
        userinit();      // first user process
        mpmain();        // finish this processor's setup
    }
}
