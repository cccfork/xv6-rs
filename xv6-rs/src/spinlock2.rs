#![allow(dead_code)]

// Mutual exclusion lock.

use core;
use core::sync::atomic::*;

use process::*;

#[repr(C)]
#[derive(Debug)]
pub struct Spinlock {
    locked: AtomicBool,     // Is the lock held?

    // For debugging:
    name: *const char,      // Name of lock.
    cpu: AtomicPtr<cpu>,    // The cpu holding the lock.
    pcs: [usize; 10],       // The call stack (an array of program counters)
    // that locked the lock.
}

impl Spinlock {
    pub fn new(name: *const char) -> Spinlock {
        Spinlock {
            locked: AtomicBool::new(false),
            name: name,
            cpu: AtomicPtr::new(core::ptr::null_mut()),
            pcs: [0; 10],
        }
    }

    // Acquire the lock.
    // Loops (spins) until the lock is acquired.
    // Holding a lock for a long time may cause
    // other CPUs to waste time spinning to acquire it.
    pub fn acquire(&self) {
        pushcli(); // disable interrupts to avoid deadlock.
        if(self.holding())
            panic("acquire");

        // The atomic_exchange_explicit is atomic.
        while atomic_exchange_explicit(&lk->locked, 1, memory_order_relaxed) {
        }

        // Tell the C compiler and the processor to not move loads or stores
        // past this point, to ensure that the critical section's memory
        // references happen after the lock is acquired.
        atomic::fence(Ordering::acquire);

        // Record info about lock acquisition for debugging.
        self.cpu.store(get_cpu(), Ordering.relaxed);
        getcallerpcs(&lk, self.pcs);
    }

    // Release the lock.
    pub fn release(&self) {
        if(!self.holding())
            panic("release");

        self.pcs[0] = 0;
        self.cpu.store(core::ptr::null_mut(), Ordering.relaxed);

        // Tell the C compiler and the processor to not move loads or stores
        // past this point, to ensure that all the stores in the critical
        // section are visible to other cores before the lock is released.
        // Both the C compiler and the hardware may re-order loads and
        // stores; memory_order_release tells them both not to.
        //
        // Release the lock, equivalent to lk->locked = 0.
        self.locked(false, Ordering.release);

        popcli();
    }

    // Record the current call stack in pcs[] by following the %ebp chain.
    fn getcallerpcs(void *v, uint pcs[])
    {
        uint *ebp;
        int i;

        ebp = (uint*)v - 2;
        for(i = 0; i < 10; i++){
            if(ebp == 0 || ebp < (uint*)KERNBASE || ebp == (uint*)0xffffffff)
                break;
            pcs[i] = ebp[1];     // saved %eip
            ebp = (uint*)ebp[0]; // saved %ebp
        }
        for(; i < 10; i++)
            pcs[i] = 0;
    }

    // Check whether this cpu is holding the lock.
    pub fn holding(&self) -> bool {
        return
            atomic_load_explicit(&lock->locked, memory_order_relaxed) &&
            atomic_load_explicit(&lock->cpu, memory_order_relaxed) == get_cpu();
    }


    // Pushcli/popcli are like cli/sti except that they are matched:
    // it takes two popcli to undo two pushcli.  Also, if interrupts
    // are off, then pushcli, popcli leaves them off.

    fn pushcli() {
        int eflags;

        eflags = readeflags();
        cli();
        if(get_cpu()->ncli == 0)
            get_cpu()->intena = eflags & FL_IF;
        get_cpu()->ncli += 1;
    }

    fn popcli() {
        if(readeflags()&FL_IF)
            panic("popcli - interruptible");
        if (--get_cpu()->ncli < 0) {
           panic("popcli");
           if(get_cpu()->ncli == 0 && get_cpu()->intena)
           sti();
        }
    }
}
