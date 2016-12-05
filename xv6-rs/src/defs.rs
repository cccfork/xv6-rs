#![allow(dead_code,
         improper_ctypes,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

pub enum void { }
pub type pde_t = u32;
pub enum buf { }
pub enum context { }
pub enum file { }
pub enum inode { }
pub enum pipe { }
pub enum proc_ { }
pub enum rtcdate { }
pub enum spinlock { }
pub enum sleeplock { }
pub enum stat { }
pub enum superblock { }

extern "C" {
    pub static mut ioapicid: u8;
    pub static mut lapic: *mut u32;
    pub static mut ismp: i32;
    pub static mut ticks: u32;
    pub static mut tickslock: spinlock;

    pub fn binit();
    pub fn bread(arg1: u32, arg2: u32) -> *mut buf;
    pub fn brelse(arg1: *mut buf);
    pub fn bwrite(arg1: *mut buf);
    pub fn consoleinit();
    pub fn cprintf(arg1: *const char, ...);
    pub fn consoleintr(getc: unsafe extern "C" fn () -> isize);
    pub fn panic(arg1: *mut char);
    pub fn exec(arg1: *mut char,
                arg2: *mut *mut char)
     -> i32;
    pub fn filealloc() -> *mut file;
    pub fn fileclose(arg1: *mut file);
    pub fn filedup(arg1: *mut file) -> *mut file;
    pub fn fileinit();
    pub fn fileread(arg1: *mut file, arg2: *mut char,
                    n: i32) -> i32;
    pub fn filestat(arg1: *mut file, arg2: *mut stat)
     -> i32;
    pub fn filewrite(arg1: *mut file, arg2: *mut char,
                     n: i32) -> i32;
    pub fn readsb(dev: i32, sb: *mut superblock);
    pub fn dirlink(arg1: *mut inode, arg2: *mut char,
                   arg3: u32) -> i32;
    pub fn dirlookup(arg1: *mut inode, arg2: *mut char,
                     arg3: *mut u32) -> *mut inode;
    pub fn ialloc(arg1: u32, arg2: i16) -> *mut inode;
    pub fn idup(arg1: *mut inode) -> *mut inode;
    pub fn iinit(dev: i32);
    pub fn ilock(arg1: *mut inode);
    pub fn iput(arg1: *mut inode);
    pub fn iunlock(arg1: *mut inode);
    pub fn iunlockput(arg1: *mut inode);
    pub fn iupdate(arg1: *mut inode);
    pub fn namecmp(arg1: *const char,
                   arg2: *const char)
     -> i32;
    pub fn namei(arg1: *mut char) -> *mut inode;
    pub fn nameiparent(arg1: *mut char,
                       arg2: *mut char) -> *mut inode;
    pub fn readi(arg1: *mut inode, arg2: *mut char,
                 arg3: u32, arg4: u32) -> i32;
    pub fn stati(arg1: *mut inode, arg2: *mut stat);
    pub fn writei(arg1: *mut inode, arg2: *mut char,
                  arg3: u32, arg4: u32) -> i32;
    pub fn ideinit();
    pub fn ideintr();
    pub fn iderw(arg1: *mut buf);
    pub fn ioapicenable(irq: i32,
                        cpu: i32);
    pub fn ioapicinit();
    pub fn kalloc() -> *mut char;
    pub fn kfree(arg1: *mut char);
    pub fn kinit1(arg1: *mut void,
                  arg2: *mut void);
    pub fn kinit2(arg1: *mut void,
                  arg2: *mut void);
    pub fn kbdintr();
    pub fn cmostime(r: *mut rtcdate);
    pub fn cpunum() -> i32;
    pub fn lapiceoi();
    pub fn lapicinit();
    pub fn lapicstartap(arg1: u8, arg2: u32);
    pub fn microdelay(arg1: i32);
    pub fn initlog(dev: i32);
    pub fn log_write(arg1: *mut buf);
    pub fn begin_op();
    pub fn end_op();
    pub fn mpinit();
    pub fn picenable(arg1: i32);
    pub fn picinit();
    pub fn pipealloc(arg1: *mut *mut file, arg2: *mut *mut file)
     -> i32;
    pub fn pipeclose(arg1: *mut pipe, arg2: i32);
    pub fn piperead(arg1: *mut pipe, arg2: *mut char,
                    arg3: i32) -> i32;
    pub fn pipewrite(arg1: *mut pipe, arg2: *mut char,
                     arg3: i32) -> i32;
    pub fn exit();
    pub fn fork() -> i32;
    pub fn growproc(arg1: i32) -> i32;
    pub fn kill(arg1: i32) -> i32;
    pub fn pinit();
    pub fn procdump();
    pub fn scheduler();
    pub fn sched();
    pub fn sleep(arg1: *mut void, arg2: *mut spinlock);
    pub fn userinit();
    pub fn wait() -> i32;
    pub fn wakeup(arg1: *mut void);
    #[link_name = "yield"]
    pub fn yield_();
    pub fn swtch(arg1: *mut *mut context, arg2: *mut context);
    pub fn acquire(arg1: *mut spinlock);
    pub fn getcallerpcs(arg1: *mut void, arg2: *mut u32);
    pub fn holding(arg1: *mut spinlock) -> i32;
    pub fn initlock(arg1: *mut spinlock, arg2: *mut char);
    pub fn release(arg1: *mut spinlock);
    pub fn pushcli();
    pub fn popcli();
    pub fn acquiresleep(arg1: *mut sleeplock);
    pub fn releasesleep(arg1: *mut sleeplock);
    pub fn holdingsleep(arg1: *mut sleeplock) -> i32;
    pub fn initsleeplock(arg1: *mut sleeplock,
                         arg2: *mut char);
    pub fn memcmp(arg1: *const void,
                  arg2: *const void, arg3: u32)
     -> i32;
    pub fn memmove(arg1: *mut void,
                   arg2: *const void, arg3: u32)
     -> *mut void;
    pub fn memset(arg1: *mut void,
                  arg2: i32, arg3: u32)
     -> *mut void;
    pub fn safestrcpy(arg1: *mut char,
                      arg2: *const char,
                      arg3: i32)
     -> *mut char;
    pub fn strlen(arg1: *const char)
     -> i32;
    pub fn strncmp(arg1: *const char,
                   arg2: *const char, arg3: u32)
     -> i32;
    pub fn strncpy(arg1: *mut char,
                   arg2: *const char,
                   arg3: i32)
     -> *mut char;
    pub fn argint(arg1: i32,
                  arg2: *mut i32) -> i32;
    pub fn argptr(arg1: i32,
                  arg2: *mut *mut char,
                  arg3: i32) -> i32;
    pub fn argstr(arg1: i32,
                  arg2: *mut *mut char)
     -> i32;
    pub fn fetchint(arg1: u32, arg2: *mut i32)
     -> i32;
    pub fn fetchstr(arg1: u32, arg2: *mut *mut char)
     -> i32;
    pub fn syscall();
    pub fn timerinit();
    pub fn idtinit();
    pub fn tvinit();
    pub fn uartinit();
    pub fn uartintr();
    pub fn uartputc(arg1: i32);
    pub fn seginit();
    pub fn kvmalloc();
    pub fn setupkvm() -> *mut pde_t;
    pub fn uva2ka(arg1: *mut pde_t, arg2: *mut char)
     -> *mut char;
    pub fn allocuvm(arg1: *mut pde_t, arg2: u32, arg3: u32)
     -> i32;
    pub fn deallocuvm(arg1: *mut pde_t, arg2: u32, arg3: u32)
     -> i32;
    pub fn freevm(arg1: *mut pde_t);
    pub fn inituvm(arg1: *mut pde_t, arg2: *mut char,
                   arg3: u32);
    pub fn loaduvm(arg1: *mut pde_t, arg2: *mut char,
                   arg3: *mut inode, arg4: u32, arg5: u32)
     -> i32;
    pub fn copyuvm(arg1: *mut pde_t, arg2: u32) -> *mut pde_t;
    pub fn switchuvm(arg1: *mut proc_);
    pub fn switchkvm();
    pub fn copyout(arg1: *mut pde_t, arg2: u32,
                   arg3: *mut void, arg4: u32)
     -> i32;
    pub fn clearpteu(pgdir: *mut pde_t, uva: *mut char);
}
