use defs::*;

pub struct LogGuard {
}

impl Drop for LogGuard {
    fn drop(&mut self) {
        unsafe {
            end_op();
        }
    }
}

pub fn lock() -> LogGuard {
    unsafe {
        begin_op();
    }
    LogGuard { }
}
