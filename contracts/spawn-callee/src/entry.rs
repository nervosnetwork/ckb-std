// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use alloc::vec;
use ckb_std::syscalls;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    let argv = ckb_std::env::argv();
    let mut std_fds: [u64; 2] = [0; 2];
    syscalls::inherited_file_descriptors(&mut std_fds);
    let mut out = vec![];
    for arg in argv {
        out.extend_from_slice(arg.to_bytes());
    }
    let len = syscalls::write(std_fds[1], &out)?;
    assert_eq!(len, 10);
    Ok(())
}
