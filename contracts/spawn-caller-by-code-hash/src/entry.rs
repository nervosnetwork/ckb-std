// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use ckb_std::ckb_constants::Source;
use ckb_std::ckb_types::core::ScriptHashType;
use ckb_std::high_level::{load_script, spawn_cell};
use ckb_std::syscalls;
use core::ffi::CStr;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    let argv = [
        CStr::from_bytes_with_nul(b"hello\0").unwrap(),
        CStr::from_bytes_with_nul(b"world\0").unwrap(),
    ];
    let mut std_fds: [u64; 2] = [0, 0];
    let mut son_fds: [u64; 3] = [0, 0, 0];
    let (r0, w0) = syscalls::pipe()?;
    std_fds[0] = r0;
    son_fds[1] = w0;
    let (r1, w1) = syscalls::pipe()?;
    std_fds[1] = w1;
    son_fds[0] = r1;
    let code_hash = load_script().unwrap().args().raw_data();
    spawn_cell(&code_hash[..], ScriptHashType::Data1, &argv, &son_fds)?;
    let mut buf: [u8; 256] = [0; 256];
    let len = syscalls::read(std_fds[0], &mut buf)?;
    assert_eq!(len, 10);
    buf[len] = 0;
    assert_eq!(
        CStr::from_bytes_until_nul(&buf).unwrap().to_str().unwrap(),
        "helloworld"
    );
    Ok(())
}
