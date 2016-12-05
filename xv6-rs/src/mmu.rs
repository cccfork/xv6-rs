#![allow(dead_code,
         non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

// This file contains definitions for the
// x86 memory management unit (MMU).

// FIXME: enum
// Eflags register
pub const FL_CF: usize = 0x00000001;        // Carry Flag
pub const FL_PF: usize = 0x00000004;        // Parity Flag
pub const FL_AF: usize = 0x00000010;        // Auxiliary carry Flag
pub const FL_ZF: usize = 0x00000040;        // Zero Flag
pub const FL_SF: usize = 0x00000080;        // Sign Flag
pub const FL_TF: usize = 0x00000100;        // Trap Flag
pub const FL_IF: usize = 0x00000200;        // Interrupt Enable
pub const FL_DF: usize = 0x00000400;        // Direction Flag
pub const FL_OF: usize = 0x00000800;        // Overflow Flag
pub const FL_IOPL_MASK: usize = 0x00003000; // I/O Privilege Level bitmask
pub const FL_IOPL_0: usize = 0x00000000;    //   IOPL == 0
pub const FL_IOPL_1: usize = 0x00001000;    //   IOPL == 1
pub const FL_IOPL_2: usize = 0x00002000;    //   IOPL == 2
pub const FL_IOPL_3: usize = 0x00003000;    //   IOPL == 3
pub const FL_NT: usize = 0x00004000;        // Nested Task
pub const FL_RF: usize = 0x00010000;        // Resume Flag
pub const FL_VM: usize = 0x00020000;        // Virtual 8086 mode
pub const FL_AC: usize = 0x00040000;        // Alignment Check
pub const FL_VIF: usize = 0x00080000;       // Virtual Interrupt Flag
pub const FL_VIP: usize = 0x00100000;       // Virtual Interrupt Pending
pub const FL_ID: usize = 0x00200000;        // ID flag

// FIXME: enum
// Control Register flags
pub const CR0_PE: usize = 0x00000001;       // Protection Enable
pub const CR0_MP: usize = 0x00000002;       // Monitor coProcessor
pub const CR0_EM: usize = 0x00000004;       // Emulation
pub const CR0_TS: usize = 0x00000008;       // Task Switched
pub const CR0_ET: usize = 0x00000010;       // Extension Type
pub const CR0_NE: usize = 0x00000020;       // Numeric Errror
pub const CR0_WP: usize = 0x00010000;       // Write Protect
pub const CR0_AM: usize = 0x00040000;       // Alignment Mask
pub const CR0_NW: usize = 0x20000000;       // Not Writethrough
pub const CR0_CD: usize = 0x40000000;       // Cache Disable
pub const CR0_PG: usize = 0x80000000;       // Paging

pub const CR4_PSE: usize = 0x00000010;      // Page size extension

// various segment selectors.
pub const SEG_KCODE: usize = 1; // kernel code
pub const SEG_KDATA: usize = 2; // kernel data+stack
pub const SEG_KCPU: usize = 3;  // kernel per-cpu data
pub const SEG_UCODE: usize = 4; // user code
pub const SEG_UDATA: usize = 5; // user data+stack
pub const SEG_TSS: usize = 6;   // this process's task state

// cpu->gdt[NSEGS] holds the above segments.
pub const NSEGS: usize = 7;

pub const PGSIZE: usize = 4096;             // bytes mapped by a page

#[inline(always)]
pub fn PGROUNDUP(sz: usize) -> usize {
    (sz + PGSIZE - 1) & !(PGSIZE - 1)
}

#[inline(always)]
pub fn PGROUNDDOWN(a: usize) -> usize {
    a & !(PGSIZE - 1)
}

// //PAGEBREAK!
// // Segment Descriptor
// #[repr(C)]
// #[derive(Copy, Clone)]
// #[derive(Debug)]
// pub struct segdesc {
//     pub _bindgen_bitfield_1_: u32,
//     pub _bindgen_bitfield_2_: u32,
//     pub _bindgen_bitfield_3_: u32,
//     pub _bindgen_bitfield_4_: u32,
//     pub _bindgen_bitfield_5_: u32,
//     pub _bindgen_bitfield_6_: u32,
//     pub _bindgen_bitfield_7_: u32,
//     pub _bindgen_bitfield_8_: u32,
//     pub _bindgen_bitfield_9_: u32,
//     pub _bindgen_bitfield_10_: u32,
//     pub _bindgen_bitfield_11_: u32,
//     pub _bindgen_bitfield_12_: u32,
//     pub _bindgen_bitfield_13_: u32,
// }
// impl ::core::default::Default for segdesc {
//     fn default() -> Self { unsafe { ::core::mem::zeroed() } }
// }
// pub type pte_t = u32;
// #[repr(C)]
// #[derive(Copy, Clone)]
// #[derive(Debug)]
// pub struct taskstate {
//     pub link: u32,
//     pub esp0: u32,
//     pub ss0: u16,
//     pub padding1: u16,
//     pub esp1: *mut u32,
//     pub ss1: u16,
//     pub padding2: u16,
//     pub esp2: *mut u32,
//     pub ss2: u16,
//     pub padding3: u16,
//     pub cr3: *mut ::std::os::raw::c_void,
//     pub eip: *mut u32,
//     pub eflags: u32,
//     pub eax: u32,
//     pub ecx: u32,
//     pub edx: u32,
//     pub ebx: u32,
//     pub esp: *mut u32,
//     pub ebp: *mut u32,
//     pub esi: u32,
//     pub edi: u32,
//     pub es: u16,
//     pub padding4: u16,
//     pub cs: u16,
//     pub padding5: u16,
//     pub ss: u16,
//     pub padding6: u16,
//     pub ds: u16,
//     pub padding7: u16,
//     pub fs: u16,
//     pub padding8: u16,
//     pub gs: u16,
//     pub padding9: u16,
//     pub ldt: u16,
//     pub padding10: u16,
//     pub t: u16,
//     pub iomb: u16,
// }
// impl ::core::default::Default for taskstate {
//     fn default() -> Self { unsafe { ::core::mem::zeroed() } }
// }
// #[repr(C)]
// #[derive(Copy, Clone)]
// #[derive(Debug)]
// pub struct gatedesc {
//     pub _bindgen_bitfield_1_: u32,
//     pub _bindgen_bitfield_2_: u32,
//     pub _bindgen_bitfield_3_: u32,
//     pub _bindgen_bitfield_4_: u32,
//     pub _bindgen_bitfield_5_: u32,
//     pub _bindgen_bitfield_6_: u32,
//     pub _bindgen_bitfield_7_: u32,
//     pub _bindgen_bitfield_8_: u32,
//     pub _bindgen_bitfield_9_: u32,
// }
// impl ::core::default::Default for gatedesc {
//     fn default() -> Self { unsafe { ::core::mem::zeroed() } }
// }
