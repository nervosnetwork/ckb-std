// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::cstr_core::{cstr, CStr};
use core::slice::from_raw_parts;

use crate::error::Error;

pub fn main(argc: u64, argv: *const *const u8) -> Result<(), Error> {
    let args = unsafe { from_raw_parts(argv, argc as usize) };

    #[cfg(target_arch = "riscv64")]
    let arg1 = unsafe { CStr::from_ptr(args[0]) };
    #[cfg(not(target_arch = "riscv64"))]
    let arg1 = unsafe { CStr::from_ptr(args[0] as *const i8) };

    #[cfg(target_arch = "riscv64")]
    let arg2 = unsafe { CStr::from_ptr(args[1]) };
    #[cfg(not(target_arch = "riscv64"))]
    let arg2 = unsafe { CStr::from_ptr(args[1] as *const i8) };

    assert_eq!(argc, 2);
    assert_eq!(arg1, cstr!("Hello World"));
    assert_eq!(arg2, cstr!("你好"));
    Ok(())
}
