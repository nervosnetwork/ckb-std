// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{ckb_constants::*, cstr_core::cstr, syscalls};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    let ret = syscalls::exec(0, Source::CellDep, 0, 0, &[arg1, arg2][..]);
    panic!("exec failed: {}", ret);
}
