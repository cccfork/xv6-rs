#![allow(unused_imports,
         non_camel_case_types)]

use defs::*;
use memlayout::*;
use mmu::*;

pub enum taskstate { }
pub enum cpu { }

// #[repr(C)]
// #[derive(Copy, Clone)]
// #[derive(Debug)]
// pub struct cpu {
//     pub apicid: u8,              // Local APIC ID
//     pub scheduler: *mut context, // swtch() here to enter scheduler
//     pub ts: taskstate,           // Used by x86 to find stack for interrupt
//     pub gdt: [segdesc; NSEGS],   // x86 global descriptor table
//     pub started: u32,            // Has the CPU started?
//     pub ncli: i32,               // Depth of pushcli nesting.
//     pub intena: i32,             // Were interrupts enabled before pushcli?

//     // Cpu-local storage variables; see below
//     pub cpu: *mut cpu,           // FIXME: don't need it
//     pub process: *mut process,   // FIXME: don't need it
// }

// extern "C" {
//     pub static cpus: [cpu; NCPU];
// }

// // &cpus[cpunum()]
// #[inline(always)]
// fn get_cpu() -> *mut cpu {
//     let mut cpu: *mut cpu;
//     asm!("movl %%gs:0, %0" : "=r"(cpu));
//     cpu
// }

// #[inline(always)]
// fn set_cpu(cpu: *mut cpu) {
//     asm!("movl %0, %%gs:0" : : "r"(cpu));
// }
