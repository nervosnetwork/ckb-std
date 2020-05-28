# ckb-std
[![Crates.io](https://img.shields.io/crates/v/ckb-std.svg)](https://crates.io/crates/ckb-std) 

This library contains serveral modules that help you write CKB contract with Rust.

## Usage

[Documentation](https://justjjy.com/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html)

### Modules

* `syscalls` module: defines [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md)
* `high_level` module: defines high level APIs
* `debug!` macro: a `println!` like macro helps debugging
* `entry!` macro: defines contract entry point
* `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust

### Memory allocator

`ckb-std` supports two memory allocators: default allocator(pure rust) and libc allocator(libc dependent).

#### Default allocator

Default allocator allocate `64K` bytes memory, a panic will occured if out of memory.

Use the macro to change the default value:

``` rust
// indicate the heap size(default heap size is 64KB, with 16B minimal memory block)
default_alloc!(64 * 1024, 16)
```

> Beware, the allocate parameters affect cycles of the contract; you should always test the contract after customizing parameters.


#### LibC allocator

To use `libc` global allocator, you must static link libc into the binary, and enable `libc` feature in this crate.

### Examples

Check `examples` and [tests](https://github.com/jjyr/ckb-std/blob/master/test/contract/src/main.rs) to learn how to use.

See also [ckb-tool](https://github.com/jjyr/ckb-tool) which helps you write tests.
