use defs::*;

pub struct InodeGuard {
    pub inode: *mut inode,
}

impl Drop for InodeGuard {
    fn drop(&mut self) {
        unsafe {
            iunlockput(self.inode);
        }
    }
}

pub fn lock(inode: *mut inode) -> InodeGuard {
    unsafe {
        ilock(inode);
    }
    InodeGuard { inode: inode }
}
