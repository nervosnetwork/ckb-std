#![no_std]
#![no_main]

use alloc::vec;
use ckb_std::{debug, default_alloc, entry};

fn main() -> i8 {
    let v = vec![0u8; 42];
    debug!("{:?}", v.len());
    0
}

entry!(main);
default_alloc!();
