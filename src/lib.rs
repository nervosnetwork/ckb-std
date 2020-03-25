#![no_std]
#![feature(asm)]

extern crate alloc;

pub mod global_alloc;
pub mod ckb_constants;
pub mod debug;
pub mod entry;
pub mod syscalls;
pub mod since;
#[cfg(feature = "libc")]
pub mod libc_alloc_wrap;
#[cfg(feature = "buddy-alloc")]
pub use buddy_alloc;
