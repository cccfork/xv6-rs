#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

// Format of an ELF executable file

pub const ELF_MAGIC: usize = 0x464C457F;  // "\x7FELF" in little endian

// File header
#[repr(C)]
#[derive(Debug)]
pub struct elfhdr {
    pub magic: usize,  // must equal ELF_MAGIC
    pub elf: [u8; 12],
    pub type_: u16,
    pub machine: u16,
    pub version: usize,
    pub entry: usize,
    pub phoff: usize,
    pub shoff: usize,
    pub flags: usize,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

impl ::core::default::Default for elfhdr {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

// Program section header
#[repr(C)]
#[derive(Debug)]
pub struct proghdr {
    pub type_: usize,
    pub off: usize,
    pub vaddr: usize,
    pub paddr: usize,
    pub filesz: usize,
    pub memsz: usize,
    pub flags: usize,
    pub align: usize,
}

impl ::core::default::Default for proghdr {
    fn default() -> Self { unsafe { ::core::mem::zeroed() } }
}

// Values for Proghdr type
pub const ELF_PROG_LOAD: usize =      1;

// Flag bits for Proghdr flags
pub const ELF_PROG_FLAG_EXEC: usize =  1;
pub const ELF_PROG_FLAG_WRITE: usize = 2;
pub const ELF_PROG_FLAG_READ: usize =  4;

//PAGEBREAK!
// Blank page.
