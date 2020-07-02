# ckb-std
[![Crates.io](https://img.shields.io/crates/v/ckb-std.svg)](https://crates.io/crates/ckb-std) 

This library contains several modules that help you write CKB contract with Rust.

## Usage

[Documentation](https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html)

### Modules

* `syscalls` module: defines [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md)
* `high_level` module: defines high level APIs
* `debug!` macro: a `println!` like macro helps debugging
* `entry!` macro: defines contract entry point
* `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust

### Memory allocator

Default allocator uses a mixed allocation strategy:

* Fixed block heap, only allocate fixed size(64B) memory block
* Dynamic memory heap, allocate any size memory block

User can invoke macro with arguments to customize the heap size. The default heap size arguments are:

(fixed heap size 4KB, dynamic heap size 64KB, dynamic heap min memory block 64B)

Use the macro with arguments to change it:

``` rust
default_alloc!(4 * 1024, 64 * 1024, 64)
```

> Beware, use difference heap size or memory block size may affect cycles of the contract; you should always test the contract after customizing parameters.

### Examples

Check `examples` and [tests](https://github.com/jjyr/ckb-std/blob/master/test/contract/src/main.rs) to learn how to use.

See also [ckb-tool](https://github.com/jjyr/ckb-tool) which helps you write tests.
