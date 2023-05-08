// Import from `core` instead of from `std` since we are in no-std mode
use crate::error::Error;
use alloc::vec;
use ckb_std::ckb_types::core::ScriptHashType;
use ckb_std::high_level::{load_script, spawn_cell};
use core::ffi::CStr;
use core::result::Result;

pub fn main() -> Result<(), Error> {
    let arg1 = CStr::from_bytes_with_nul(b"hello\0").unwrap();
    let arg2 = CStr::from_bytes_with_nul(b"world\0").unwrap();
    let code_hash = load_script().unwrap().args().raw_data();
    ckb_std::debug!("code_hash: {:?}", code_hash);

    let mut content = vec![0; 80];
    let ret = spawn_cell(
        &code_hash[..],
        ScriptHashType::Data1,
        &[arg1, arg2][..],
        8,
        &mut content,
    );
    assert!(ret == Ok(0));
    assert!(content.len() == 10);
    content.resize(content.len() + 1, 0);
    let c_str = CStr::from_bytes_until_nul(&content).unwrap();
    assert_eq!(c_str.to_str().unwrap(), "helloworld");
    Ok(())
}
