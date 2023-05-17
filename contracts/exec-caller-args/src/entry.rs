// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    ckb_types::core::ScriptHashType,
    high_level::{self, load_script},
};

use crate::error::Error;

pub fn main() -> Result<(), Error> {
    let args = [42, 0, 0, 0, 42, 0];
    let code_hash = load_script().unwrap().args().raw_data();
    ckb_std::debug!("code_hash: {:?}", code_hash);
    let ret = high_level::exec_cell_with_args(&code_hash[..], ScriptHashType::Data1, 0, 0, &args)
        .unwrap();
    panic!("exec failed: {}", ret);
}
