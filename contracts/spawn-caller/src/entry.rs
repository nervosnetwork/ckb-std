// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use ckb_std::ckb_constants::Source;
use ckb_std::syscalls;
use core::ffi::CStr;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    let arg1 = CStr::from_bytes_with_nul(b"hello\0").unwrap();
    let arg2 = CStr::from_bytes_with_nul(b"world\0").unwrap();

    let mut spgs_exit_code: i8 = 0;
    let mut spgs_content = [0u8; 80];
    let mut spgs_content_length: u64 = 80;
    let spgs = syscalls::SpawnArgs {
        memory_limit: 8,
        exit_code: &mut spgs_exit_code as *mut i8,
        content: &mut spgs_content as *mut u8,
        content_length: &mut spgs_content_length as *mut u64,
    };
    let ret = syscalls::spawn(1, Source::CellDep, 0, &[arg1, arg2][..], &spgs);
    assert!(ret == 0);
    assert!(spgs_exit_code == 0);
    let c_str = CStr::from_bytes_until_nul(&spgs_content).unwrap();
    assert_eq!(c_str.to_str().unwrap(), "helloworld");
    Ok(())
}
