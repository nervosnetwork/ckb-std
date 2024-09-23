#![no_std]
#![no_main]

use ckb_std::type_id::check_type_id;
use ckb_std::{default_alloc, entry};

entry!(main);
default_alloc!();

fn main() -> i8 {
    match check_type_id(0) {
        Ok(_) => 0,
        Err(_) => -10,
    }
}
