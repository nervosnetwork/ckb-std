// Import from `core` instead of from `std` since we are in no-std mode
use core::ffi::CStr;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_types::core::ScriptHashType,
    high_level::{self, load_script},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let arg1 = CStr::from_bytes_with_nul(b"Hello World\0").unwrap();
    let arg2 = CStr::from_bytes_with_nul("你好\0".as_bytes()).unwrap();
    let code_hash = load_script().unwrap().args().raw_data();
    ckb_std::debug!("code_hash: {:?}", code_hash);
    high_level::exec_cell(&code_hash[..], ScriptHashType::Data1, &[arg1, arg2][..])?;
    unreachable!()
}
