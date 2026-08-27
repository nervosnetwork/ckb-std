# ckb-std Developer Guide
This document provides a guide for developers and AI agents on how to use ckb-std.

## Compilation
All projects with ckb-std should target RISC-V (rv64imcb) and require no_std.

The recommended compilation options for Rust are as follows:
```
-C target-feature=+zba,+zbb,+zbc,+zbs -C passes=lower-atomic --target=riscv64imac-unknown-none-elf --release -C debug-assertions
```

This is already specified by the scaffold [ckb-script-templates](https://github.com/nervosnetwork/ckb-script-templates).

## Features

* calc-hash, supports the blake2b hash function
* allocator, a basic memory allocator
* native-simulator, the native simulator of ckb-std that lets on-chain scripts run on native machines
* dlopen-c, dynamic library support on ckb-vm
* build-with-clang, uses clang as the compiler when C is used
* libc, for compiling with C. Normally works with `dlopen-c`.
* dummy-atomic, a stub for atomic operations
* log, supports logging
* type-id, Type ID support

Some notes:
* `dlopen-c` is to support dynamic libraries on ckb-vm. However, it is not needed nowadays, since this can be done with the `exec` or `spawn` syscalls. Thus, no C compiler is needed, and the `libc` and `build-with-clang` features can be disabled.
* `dummy-atomic` works with the Rust `-C target-feature-a` option. Since we now have `-C passes=lower-atomic`, it is also not needed.

The recommended feature combination is `allocator`, `calc-hash`, and `ckb-types`. Add more as required.

## Syscalls
Detailed information about syscalls:
* https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md
* https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0034-vm-syscalls-2/0034-vm-syscalls-2.md
* https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0050-vm-syscalls-3/0050-vm-syscalls-3.md

When using syscalls, first use the `high_level` module. If it doesn't meet your requirements, use the syscalls module instead. When using syscalls, use the predefined types in ckb-types, such as Script, WitnessArgs, Transaction, CellInput, CellOutput, etc, instead of parsing molecule from scratch.

IMPORTANT: Overview of transaction structure: https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0022-transaction-structure/0022-transaction-structure.md

When the `stub-syscalls` feature is enabled, these syscalls can be mocked, which is useful for unit tests and fuzzing tests.

## Log
Logging is supported by enabling the `log` feature. Call `logger::init` in the entry first, and then call the logger functions.

It is the most useful debugging method for AI agents. Enable `log` by default, and add log statements as required.
When troubleshooting, add logs to suspicious locations first and then verify.
Some libraries might need the environment variable `RUST_LOG=info` to enable outputs. When using `log`, the first step is to make sure the log can be output properly.

## Others
Like a traditional C program, on-chain scripts accept argc/argv via the `env` module.

Use the `asserts` module if the failure is unrecoverable. 