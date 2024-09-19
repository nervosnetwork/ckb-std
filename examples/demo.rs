#![no_std]
#![no_main]

use alloc::vec;
use ckb_std;

ckb_std::default_alloc!();
ckb_std::entry!(program_entry);

fn program_entry() -> i8 {
    let v = vec![0u8; 42];
    ckb_std::debug!("{:?}", v.len());
    0
}
