# ckb-contract-std
[![Crates.io](https://img.shields.io/crates/v/ckb-contract-std.svg)](https://crates.io/crates/ckb-contract-std)

This library contains serveral modules that could help you write CKB contract with Rust.

* syscall module: defines [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md) functions
* debug macro: a `println!` like macro helps debugging
* setup macro: defines contract entry point

Check `examples`, [docs](https://docs.rs/crate/ckb-contract-std) and this [tutorial](https://justjjy.com/Build-CKB-contract-with-Rust-part-1) to learn how to use.

See also [ckb-contract-tool](https://github.com/jjyr/ckb-contract-tool) which helps you write tests.
