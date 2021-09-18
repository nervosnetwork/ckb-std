#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

use ckb_std::{ckb_types::core::ScriptHashType, cstr_core::cstr, default_alloc, entry, high_level};

#[no_mangle]
pub fn main() -> i8 {
    let arg1 = cstr!("Hello World");
    let arg2 = cstr!("你好");
    // $ ckb-cli util blake2b --binary-path target/riscv64imac-unknown-none-elf/debug/exec-callee
    // 0xf1868a555b7cbec49ff8f7665260ca036d24baffba7a5d765d464e7b127a5d97
    let code_hash = [
        0xf1, 0x86, 0x8a, 0x55, 0x5b, 0x7c, 0xbe, 0xc4, 0x9f, 0xf8, 0xf7, 0x66, 0x52, 0x60, 0xca,
        0x03, 0x6d, 0x24, 0xba, 0xff, 0xba, 0x7a, 0x5d, 0x76, 0x5d, 0x46, 0x4e, 0x7b, 0x12, 0x7a,
        0x5d, 0x97,
    ];
    let ret = high_level::exec_cell(
        &code_hash[..],
        ScriptHashType::Data1,
        0,
        0,
        &[arg1, arg2][..],
    )
    .unwrap();
    panic!("exec failed: {}", ret);
}

entry!(main);
default_alloc!();
