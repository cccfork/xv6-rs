#![allow(unused_assignments)]


#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let ret: u8;
    asm!("inb %dx, %al"
         : "={ax}"(ret)
         : "{dx}"(port)
         :
         : "volatile");
    ret
}

#[inline(always)]
pub unsafe fn insl(port: u16, addr: usize, cnt: usize) {
    asm!("cld; rep insl %dx, (%edi)"
         :
         : "{ecx}"(cnt), "{dx}"(port), "{edi}"(addr)
         : "ecx", "edi", "memory", "cc"
         : "volatile");
}

#[inline(always)]
pub unsafe fn outb(port: u16, val: u8) {
    asm!("outb %al, %dx"
         :
         : "{dx}"(port), "{al}"(val)
         :
         : "volatile");
}

#[inline(always)]
pub unsafe fn outw(port: u16, val: u16) {
    asm!("outw %ax, %dx"
         :
         : "{dx}"(port), "{al}"(val)
         :
         :
         : "volatile");
}

#[inline(always)]
pub unsafe fn outsl(port: u16, addr: usize, cnt: usize) {
    asm!("cld; rep outsl (%esi), %dx"
         :
         : "{ecx}"(cnt), "{dx}"(port), "{esi}"(addr)
         : "ecx", "edi", "cc"
         : "volatile");
}

#[inline(always)]
pub unsafe fn stosb(mut addr: usize, data: usize, mut cnt: usize) {
    asm!("cld; rep stosb"
         : "={edi}"(addr), "={ecx}"(cnt)
         : "{edi}"(addr), "{ecx}"(cnt), "{eax}"(data)
         : "ecx", "edi", "memory", "cc"
         : "volatile");
}


#[inline(always)]
pub unsafe fn cli() {
    asm!("cli"
         :
         :
         :
         : "volatile");
}
