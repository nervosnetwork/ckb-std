//! ckb-std
//!
//! # Modules
//!
//! * `high_level` module: defines high level syscall API
//! * `syscalls` module: defines low level [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md)
//! * `debug!` macro: a `println!` like macro helps debugging
//! * `entry!` macro: defines contract entry point
//! * `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust

#![cfg_attr(not(feature = "native-simulator"), no_std)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

extern crate alloc;

pub mod ckb_constants;
#[doc(hidden)]
pub mod debug;
#[doc(hidden)]
pub mod entry;
pub mod env;
pub mod error;
#[doc(hidden)]
pub mod global_alloc_macro;
#[cfg(feature = "ckb-types")]
pub mod high_level;
pub mod since;
pub mod syscalls;

#[cfg(feature = "ckb-types")]
pub use ckb_types;
#[cfg(feature = "ckb-types")]
pub mod dynamic_loading;
#[cfg(all(target_arch = "riscv64", feature = "dlopen-c"))]
pub mod dynamic_loading_c_impl;
#[cfg(feature = "allocator")]
pub use buddy_alloc;
#[cfg(feature = "dummy-atomic")]
pub mod dummy_atomic;
#[cfg(feature = "log")]
pub mod logger;
#[cfg(feature = "log")]
pub use log;
