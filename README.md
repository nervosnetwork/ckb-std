# ckb-std
[![Crates.io](https://img.shields.io/crates/v/ckb-std.svg)](https://crates.io/crates/ckb-std)

This library contains serveral modules that could help you write CKB contract with Rust.

## Usage

* syscalls module: defines [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md) functions
* `debug!` macro: a `println!` like macro helps debugging
* `entry!` macro: defines contract entry point
* `default_alloc!` and `libc_alloc!` macro: defines global allocator

To use `libc` global allocator, you must static link libc into the binary, and enable `libc` feature in this crate.

Check `examples`, [tests](https://github.com/jjyr/ckb-std/blob/master/test/contract/src/main.rs), [docs](https://docs.rs/crate/ckb-std) and this [tutorial](https://justjjy.com/Build-CKB-contract-with-Rust-part-1) to learn how to use.

See also [ckb-tool](https://github.com/jjyr/ckb-tool) which helps you write tests.
