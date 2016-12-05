use core;
use defs::*;
use param;
use mmu;
use elf;
use log;
use fs;

pub struct PgdirGuard {
    pub pgdir: *mut pde_t,
}

impl Drop for PgdirGuard {
    fn drop(&mut self) {
        unsafe {
            freevm(self.pgdir);
        }
    }
}

impl PgdirGuard {
    pub fn lock(pgdir: *mut pde_t) -> PgdirGuard {
        PgdirGuard { pgdir: pgdir }
    }
}

#[no_mangle]
pub unsafe extern "C" fn exec(path: *const char, argv: *const *const char) -> isize {
    let pgdir = setupkvm();
    if pgdir.is_null() {
        return -1;
    }
    let pgdir_guard = PgdirGuard::lock(pgdir);

    let mut elf = elf::elfhdr::default();
    let sz = exec_load(path, pgdir, &mut elf);
    if sz < 0 {
        return -1;
    }

    // Allocate two pages at the next page boundary.
    // Make the first inaccessible.  Use the second as the user stack.
    let sz = mmu::PGROUNDUP(sz as usize);
    let sz = allocuvm(pgdir, sz, sz + 2 * mmu::PGSIZE);
    if sz == 0 {
        return -1;
    }
    clearpteu(pgdir, (sz - 2 * mmu::PGSIZE) as *mut char);
    let mut sp = sz;

    // Push argument strings, prepare rest of stack in ustack.
    let mut ustack = [0; 3 + param::MAXARG + 1];
    let mut argc = 0;
    while (*argv.offset(argc as isize)).is_null() {
        if argc >= param::MAXARG {
            return -1;
        }
        sp = (sp - (strlen(*argv.offset(argc as isize)) + 1)) & !3;
        if copyout(pgdir, sp, *argv.offset(argc as isize) as *const void, strlen(*argv.offset(argc as isize)) + 1) < 0 {
            return -1;
        }
        ustack[3 + argc] = sp;

        argc += 1;
    }
    ustack[3 + argc] = 0;

    ustack[0] = 0xffffffff;  // fake return PC
    ustack[1] = argc;
    ustack[2] = sp - (argc + 1) * 4;  // argv pointer

    sp -= (3 + argc + 1) * 4;
    if copyout(pgdir, sp, &ustack as *const usize as *const void, (3 + argc + 1) * 4) < 0 {
        return -1;
    }

    // Save program name for debugging.
    let last = strrchr(path, '/');
    exec_init(last, pgdir, sz, &elf as *const elf::elfhdr, sp);

    core::mem::forget(pgdir_guard);
    0
}

unsafe fn exec_load(path: *const char, pgdir: *mut pde_t, elf: &mut elf::elfhdr) -> isize {
    let _ = log::lock();

    let ip = namei(path);
    if ip.is_null() {
        return -1;
    }
    let _ = fs::lock(ip);

    // Check ELF header
    let mut ph = elf::proghdr::default();
    if readi(ip, elf as *mut elf::elfhdr as *mut char, 0, core::mem::size_of_val(&elf)) < core::mem::size_of_val(&elf) {
        return -1;
    }
    if elf.magic != elf::ELF_MAGIC {
        return -1;
    }

    // Load program into memory.
    let mut sz = 0;
    let mut off = elf.phoff;
    for _ in 0..elf.phnum {
        if readi(ip, &mut ph as *mut elf::proghdr as *mut char, off, core::mem::size_of_val(&ph)) != core::mem::size_of_val(&ph) {
            return -1;
        }
        if ph.type_ != elf::ELF_PROG_LOAD {
            continue;
        }
        if ph.memsz < ph.filesz {
            return -1;
        }
        if ph.vaddr + ph.memsz < ph.vaddr {
            return -1;
        }
        sz = allocuvm(pgdir, sz, ph.vaddr + ph.memsz);
        if sz == 0 {
            return -1;
        }
        if ph.vaddr % mmu::PGSIZE != 0 {
            return -1;
        }
        if loaduvm(pgdir, ph.vaddr as *mut char, ip, ph.off, ph.filesz) < 0 {
            return -1;
        }

        off += core::mem::size_of_val(&ph);
    }

    sz as isize
}
