// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let args = ckb_std::env::args();
    ckb_std::debug!("argv: {:?}", args);
    assert_eq!(&[42, 0, 0, 0, 42, 0], args.as_slice());
    Ok(())
}
