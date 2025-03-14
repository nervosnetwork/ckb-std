///
/// Some Rust code can be compiled into atomic instructions for the RISC-V
/// target. However, these atomic instructions are not supported on ckb-vm. To
/// address this issue, this module has been introduced.
///
/// This library provides a Rust dummy atomic implementation inspired by
/// [xxuejie/lib-dummy-atomics](https://github.com/xxuejie/lib-dummy-atomics).
///
/// When the RISC-V atomic extension is disabled by specifying the
/// `target-feature=-a` flag, LLVM will attempt to link the atomic operations to
/// functions prefixed with `__atomic` in this module. For more details, refer
/// to the [LLVM Atomics Documentation](https://llvm.org/docs/Atomics.html).
///
/// On the CKB-VM, only a single thread is present, making dummy atomic
/// operations sufficient for its purposes.
///
use core::cmp::PartialEq;
use core::ffi::c_void;
use core::ops::{Add, BitAnd, BitOr, BitXor, Not, Sub};

fn atomic_exchange<T>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let p = ptr as *mut T;
        p.replace(val)
    }
}

fn atomic_compare_exchange<T: PartialEq>(
    ptr: *mut c_void,
    expected: *mut c_void,
    desired: T,
) -> bool {
    unsafe {
        let dst = ptr as *mut T;
        let old = expected as *mut T;
        let dst_val: T = dst.read();
        if dst_val == old.read() {
            dst.write(desired);
            true
        } else {
            old.write(dst_val);
            false
        }
    }
}

fn atomic_fetch_add<T: Add<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(old + val);
        old
    }
}

fn atomic_fetch_sub<T: Sub<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(old - val);
        old
    }
}

fn atomic_fetch_xor<T: BitXor<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(old ^ val);
        old
    }
}

fn atomic_fetch_and<T: BitAnd<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(old & val);
        old
    }
}

fn atomic_fetch_or<T: BitOr<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(old | val);
        old
    }
}

fn atomic_fetch_nand<T: BitAnd<Output = T> + Not<Output = T> + Copy>(
    ptr: *mut c_void,
    val: T,
) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        dst.write(!(old & val));
        old
    }
}

fn atomic_add_fetch<T: Add<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = old + val;
        dst.write(val2);
        val2
    }
}

fn atomic_sub_fetch<T: Sub<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = old - val;
        dst.write(val2);
        val2
    }
}

fn atomic_xor_fetch<T: BitXor<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = old ^ val;
        dst.write(val2);
        val2
    }
}

fn atomic_and_fetch<T: BitAnd<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = old & val;
        dst.write(val2);
        val2
    }
}

fn atomic_or_fetch<T: BitOr<Output = T> + Copy>(ptr: *mut c_void, val: T) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = old | val;
        dst.write(val2);
        val2
    }
}

fn atomic_nand_fetch<T: BitAnd<Output = T> + Not<Output = T> + Copy>(
    ptr: *mut c_void,
    val: T,
) -> T {
    unsafe {
        let dst = ptr as *mut T;
        let old: T = dst.read();
        let val2 = !(old & val);
        dst.write(val2);
        val2
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_exchange_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_exchange(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_exchange_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_exchange(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_exchange_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_exchange(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_exchange_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_exchange(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_compare_exchange_1(
    ptr: *mut c_void,
    expected: *mut c_void,
    desired: u8,
    _weak: bool,
    _success_memorder: isize,
    _failure_memorder: isize,
) -> bool {
    atomic_compare_exchange(ptr, expected, desired)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_compare_exchange_2(
    ptr: *mut c_void,
    expected: *mut c_void,
    desired: u16,
    _weak: bool,
    _success_memorder: isize,
    _failure_memorder: isize,
) -> bool {
    atomic_compare_exchange(ptr, expected, desired)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_compare_exchange_4(
    ptr: *mut c_void,
    expected: *mut c_void,
    desired: u32,
    _weak: bool,
    _success_memorder: isize,
    _failure_memorder: isize,
) -> bool {
    atomic_compare_exchange(ptr, expected, desired)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_compare_exchange_8(
    ptr: *mut c_void,
    expected: *mut c_void,
    desired: u64,
    _weak: bool,
    _success_memorder: isize,
    _failure_memorder: isize,
) -> bool {
    atomic_compare_exchange(ptr, expected, desired)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_1(ptr: *const c_void, _memorder: isize) -> u8 {
    unsafe {
        let p = ptr as *mut u8;
        p.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_2(ptr: *const c_void, _memorder: isize) -> u16 {
    unsafe {
        let p = ptr as *mut u16;
        p.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_4(ptr: *const c_void, _memorder: isize) -> u32 {
    unsafe {
        let p = ptr as *mut u32;
        p.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_load_8(ptr: *const c_void, _memorder: isize) -> u64 {
    unsafe {
        let p = ptr as *mut u64;
        p.read()
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_store_1(ptr: *mut c_void, val: u8, _memorder: isize) {
    unsafe {
        let p = ptr as *mut u8;
        p.write(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_store_2(ptr: *mut c_void, val: u16, _memorder: isize) {
    unsafe {
        let p = ptr as *mut u16;
        p.write(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_store_4(ptr: *mut c_void, val: u32, _memorder: isize) {
    unsafe {
        let p = ptr as *mut u32;
        p.write(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_store_8(ptr: *mut c_void, val: u64, _memorder: isize) {
    unsafe {
        let p = ptr as *mut u64;
        p.write(val);
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_add_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_add(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_add_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_add(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_add_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_add(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_add_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_add(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_sub_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_sub(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_sub_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_sub(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_sub_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_sub(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_sub_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_sub(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_and_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_and(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_and_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_and(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_and_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_and(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_and_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_and(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_xor_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_xor(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_xor_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_xor(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_xor_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_xor(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_xor_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_xor(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_or_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_or(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_or_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_or(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_or_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_or(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_or_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_or(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_nand_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_fetch_nand(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_nand_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_fetch_nand(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_nand_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_fetch_nand(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_fetch_nand_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_fetch_nand(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_add_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_add_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_add_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_add_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_add_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_add_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_add_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_add_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_sub_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_sub_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_sub_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_sub_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_sub_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_sub_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_sub_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_sub_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_and_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_and_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_and_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_and_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_and_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_and_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_and_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_and_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_xor_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_xor_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_xor_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_xor_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_xor_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_xor_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_xor_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_xor_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_or_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_or_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_or_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_or_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_or_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_or_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_or_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_or_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_nand_fetch_1(ptr: *mut c_void, val: u8, _memorder: isize) -> u8 {
    atomic_nand_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_nand_fetch_2(ptr: *mut c_void, val: u16, _memorder: isize) -> u16 {
    atomic_nand_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_nand_fetch_4(ptr: *mut c_void, val: u32, _memorder: isize) -> u32 {
    atomic_nand_fetch(ptr, val)
}

#[unsafe(no_mangle)]
pub extern "C" fn __atomic_nand_fetch_8(ptr: *mut c_void, val: u64, _memorder: isize) -> u64 {
    atomic_nand_fetch(ptr, val)
}
