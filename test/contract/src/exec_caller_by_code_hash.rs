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
    // 0x6e8b98ac9f53c330ee513cee096973371f914e6bc297b254366de66e09a83408
    let code_hash = [
        0x6e, 0x8b, 0x98, 0xac, 0x9f, 0x53, 0xc3, 0x30, 0xee, 0x51, 0x3c, 0xee, 0x09, 0x69, 0x73,
        0x37, 0x1f, 0x91, 0x4e, 0x6b, 0xc2, 0x97, 0xb2, 0x54, 0x36, 0x6d, 0xe6, 0x6e, 0x09, 0xa8,
        0x34, 0x08,
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
