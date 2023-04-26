// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use alloc::vec;
use ckb_std::syscalls;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    assert_eq!(syscalls::get_memory_limit(), 8);
    let argv = ckb_std::env::argv();
    ckb_std::debug!("argv: {:?}", argv);
    let mut out = vec![];
    for arg in argv {
        out.extend_from_slice(arg.to_bytes());
    }
    syscalls::set_content(&out)?;
    Ok(())
}
