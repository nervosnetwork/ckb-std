use alloc::vec;
use alloc::vec::Vec;
use blake2b_ref::{Blake2b, Blake2bBuilder};
#[allow(unused_imports)]
use ckb_std::{
    ckb_constants::*, ckb_types::core::ScriptHashType, ckb_types::packed, ckb_types::prelude::*,
    debug, error::SysError, high_level, syscalls,
};
use core::mem::size_of;

#[cfg(target_arch = "riscv64")]
use crate::code_hashes::CODE_HASH_SHARED_LIB;
use crate::error::Error;
#[cfg(target_arch = "riscv64")]
use bytes;
#[cfg(target_arch = "riscv64")]
use ckb_std::dummy_atomic;
use ckb_std::since::{EpochNumberWithFraction, Since};
#[cfg(target_arch = "riscv64")]
use ckb_std::{dynamic_loading, dynamic_loading_c_impl};
#[cfg(target_arch = "riscv64")]
use core::ffi::c_void;
#[cfg(target_arch = "riscv64")]
use core::mem::size_of_val;
#[cfg(target_arch = "riscv64")]
use lazy_static::lazy_static;
#[cfg(target_arch = "riscv64")]
use spin::Mutex;

fn new_blake2b() -> Blake2b {
    const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
    Blake2bBuilder::new(32)
        .personal(CKB_HASH_PERSONALIZATION)
        .build()
}

fn test_basic() {
    let v = vec![0u8; 42];
    debug!("{:?}", v.len());
}

fn test_load_data() {
    let data = high_level::load_cell_data(0, Source::Output).unwrap();
    assert_eq!(data.len(), 1000);
    assert!(data.iter().all(|&b| b == 42));
}

fn test_load_cell_field() {
    let mut buf = [0u8; size_of::<u64>()];
    let len = syscalls::load_cell_by_field(&mut buf, 0, 0, Source::GroupInput, CellField::Capacity)
        .unwrap();
    assert_eq!(len, buf.len());
    let capacity = u64::from_le_bytes(buf);
    let capacity2 = high_level::load_cell_capacity(0, Source::GroupInput).unwrap();
    assert_eq!(capacity, capacity2);
    debug!("input capacity {}", capacity);
}

fn test_load_tx_hash() {
    let mut tx_hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
    assert_eq!(len, tx_hash.len());
    let tx_hash2 = high_level::load_tx_hash().unwrap();
    assert_eq!(&tx_hash, &tx_hash2);
    debug!("tx hash {:?}", tx_hash);
}

fn test_partial_load_tx_hash() {
    let mut tx_hash = [0u8; 32];
    let len = syscalls::load_tx_hash(&mut tx_hash, 0).unwrap();
    assert_eq!(len, tx_hash.len());
    assert_ne!(tx_hash, [0u8; 32]);

    // partial load ..16
    let mut buf = [0u8; 16];
    let err = syscalls::load_tx_hash(&mut buf, 0).unwrap_err();
    assert_eq!(err, SysError::LengthNotEnough(32));
    assert_eq!(buf[..], tx_hash[..16]);
    // partial load 16..
    let len = syscalls::load_tx_hash(&mut buf, 16).unwrap();
    assert_eq!(len, buf.len());
    assert_eq!(buf[..], tx_hash[16..]);
}

fn test_high_level_apis() {
    use high_level::*;

    let tx = load_transaction().unwrap();
    let output = load_cell(0, Source::Output).unwrap();
    let output2 = tx.raw().outputs().get(0).unwrap();
    assert_eq!(output.as_slice(), output2.as_slice());

    let script = load_script().unwrap();
    let lock_script = load_cell_lock(0, Source::Input).unwrap();
    assert_eq!(script.as_slice(), lock_script.as_slice());

    let lock_hash = load_cell_lock_hash(0, Source::Input).unwrap();
    let lock_hash2 = {
        let mut buf = [0u8; 32];
        let mut hasher = new_blake2b();
        hasher.update(lock_script.as_slice());
        hasher.finalize(&mut buf);
        buf
    };
    assert_eq!(lock_hash, lock_hash2);

    let tx_hash = load_tx_hash().unwrap();
    let tx_hash2 = {
        let mut buf = [0u8; 32];
        let mut hasher = new_blake2b();
        hasher.update(tx.raw().as_slice());
        hasher.finalize(&mut buf);
        buf
    };
    assert_eq!(tx_hash, tx_hash2);

    let inputs_capacity = QueryIter::new(load_cell, Source::Input)
        .map(|cell| {
            let capacity: u64 = cell.capacity().unpack();
            capacity
        })
        .sum::<u64>();
    let outputs_capacity = QueryIter::new(load_cell, Source::Output)
        .map(|cell| {
            let capacity: u64 = cell.capacity().unpack();
            capacity
        })
        .sum::<u64>();
    assert_eq!(inputs_capacity, outputs_capacity);
}

fn test_query() {
    use high_level::*;

    let outputs: Vec<_> = QueryIter::new(load_cell, Source::Output).collect();
    assert_eq!(outputs.len(), 2);

    let inputs: Vec<_> = QueryIter::new(load_input, Source::Input).collect();
    assert_eq!(inputs.len(), 1);

    let cell_deps: Vec<_> = QueryIter::new(load_cell, Source::CellDep).collect();
    assert_eq!(cell_deps.len(), 2);

    let header_deps: Vec<_> = QueryIter::new(load_header, Source::HeaderDep).collect();
    assert_eq!(header_deps.len(), 0);

    let witnesses: Vec<_> = QueryIter::new(load_witness_args, Source::Input).collect();
    assert_eq!(witnesses.len(), 0);

    let type_hashes: Option<Vec<_>> =
        QueryIter::new(load_cell_type_hash, Source::Input).collect::<Option<_>>();
    assert!(type_hashes.is_none());

    let type_scripts: Option<Vec<_>> =
        QueryIter::new(load_cell_type, Source::Input).collect::<Option<_>>();
    assert!(type_scripts.is_none());
}

fn test_calc_data_hash() {
    let data = high_level::load_cell_data(0, Source::Output).unwrap();
    let data_hash = packed::CellOutput::calc_data_hash(&data);
    debug!("data hash {:?}", data_hash);
}

#[cfg(target_arch = "riscv64")]
type ContextTypeOld = dynamic_loading::CKBDLContext<[u8; 64 * 1024]>;

#[cfg(target_arch = "riscv64")]
fn test_dynamic_loading(context: &mut ContextTypeOld) {
    unsafe {
        #[allow(deprecated)]
        let lib = context
            .load(&CODE_HASH_SHARED_LIB)
            .expect("load shared lib");
        type Plus42 = unsafe extern "C" fn(n: usize) -> usize;
        let plus_42: dynamic_loading::Symbol<Plus42> = lib.get(b"plus_42").expect("find plus_42");
        assert_eq!(plus_42(13), 13 + 42);

        let foo: dynamic_loading::Symbol<Foo> = lib.get(b"foo").expect("find foo");
        let ptr = foo();
        let mut buf = [0u8; 3];
        buf.as_mut_ptr().copy_from(ptr, buf.len());
        assert_eq!(&buf[..], b"foo");

        // load multiple libraries
        let mut size = size_of_val(context) - lib.consumed_size();
        let mut offset = lib.consumed_size();
        let mut libs = Vec::new();

        for _i in 0..3 {
            #[allow(deprecated)]
            let lib = context
                .load_with_offset(&CODE_HASH_SHARED_LIB, offset, size)
                .expect("load shared lib");
            size -= lib.consumed_size();
            offset += lib.consumed_size();
            libs.push(lib);
        }

        type Foo = unsafe extern "C" fn() -> *const u8;

        for lib in libs {
            let plus_42: dynamic_loading::Symbol<Plus42> =
                lib.get(b"plus_42").expect("find plus_42");
            assert_eq!(plus_42(13), 13 + 42);

            let foo: dynamic_loading::Symbol<Foo> = lib.get(b"foo").expect("find foo");
            let ptr = foo();
            let mut buf = [0u8; 3];
            buf.as_mut_ptr().copy_from(ptr, buf.len());
            assert_eq!(&buf[..], b"foo");
        }
    }
}

#[cfg(target_arch = "riscv64")]
type ContextType = dynamic_loading_c_impl::CKBDLContext<[u8; 64 * 1024]>;

#[cfg(target_arch = "riscv64")]
fn test_dynamic_loading_c_impl(context: &mut ContextType) {
    unsafe {
        #[allow(deprecated)]
        let lib = context
            .load(&CODE_HASH_SHARED_LIB)
            .expect("load shared lib");
        type Plus42 = unsafe extern "C" fn(n: usize) -> usize;
        let plus_42: dynamic_loading_c_impl::Symbol<Plus42> =
            lib.get(b"plus_42").expect("find plus_42");
        assert_eq!(plus_42(13), 13 + 42);

        let foo: dynamic_loading_c_impl::Symbol<Foo> = lib.get(b"foo").expect("find foo");
        let ptr = foo();
        let mut buf = [0u8; 3];
        buf.as_mut_ptr().copy_from(ptr, buf.len());
        assert_eq!(&buf[..], b"foo");

        // load multiple libraries
        let mut size = size_of_val(context) - lib.consumed_size();
        let mut offset = lib.consumed_size();
        let mut libs = Vec::new();

        for _i in 0..3 {
            let lib = context
                .load_with_offset(&CODE_HASH_SHARED_LIB, ScriptHashType::Data, offset, size)
                .expect("load shared lib");
            size -= lib.consumed_size();
            offset += lib.consumed_size();
            libs.push(lib);
        }

        type Foo = unsafe extern "C" fn() -> *const u8;

        for lib in libs {
            let plus_42: dynamic_loading_c_impl::Symbol<Plus42> =
                lib.get(b"plus_42").expect("find plus_42");
            assert_eq!(plus_42(13), 13 + 42);

            let foo: dynamic_loading_c_impl::Symbol<Foo> = lib.get(b"foo").expect("find foo");
            let ptr = foo();
            let mut buf = [0u8; 3];
            buf.as_mut_ptr().copy_from(ptr, buf.len());
            assert_eq!(&buf[..], b"foo");
        }
    }
}

fn test_vm_version() {
    let version = syscalls::vm_version().unwrap();
    debug!("vm version: {}", version);
    // currently, version 1(before hardfork) and 2(after hardfork) are both ok
    assert!(version == 1 || version == 2);
}

fn test_current_cycles() {
    let cycles = syscalls::current_cycles();
    debug!("current cycles: {}", cycles);
    assert!(cycles > 300);
}

fn test_since() {
    assert_eq!(
        Since::from_block_number(0x12300, true),
        Some(Since::new(0x12300))
    );
    assert_eq!(
        Since::from_block_number(0x12300, false),
        Some(Since::new(0x8000_0000_0001_2300))
    );
    assert_eq!(Since::from_block_number(0x1100_0000_0000_2300, true), None);
    assert_eq!(
        Since::from_timestamp(0xffaa_1122, true),
        Some(Since::new(0x4000_0000_ffaa_1122))
    );
    assert_eq!(
        Since::from_timestamp(0xffaa_1122, false),
        Some(Since::new(0xc000_0000_ffaa_1122))
    );
    assert_eq!(Since::from_timestamp(0x0100_0000_ffaa_1122, false), None);
    assert_eq!(
        Since::from_epoch(EpochNumberWithFraction::from_full_value(1), true),
        Since::new(0x2000_0100_0000_0001)
    );
    assert_eq!(
        Since::from_epoch(EpochNumberWithFraction::from_full_value(1), false),
        Since::new(0xa000_0100_0000_0001)
    );
    assert_eq!(EpochNumberWithFraction::create(16777216, 1, 1000), None,);
    assert_eq!(EpochNumberWithFraction::create(10000, 0, 0), None,);
    assert_eq!(EpochNumberWithFraction::create(10000, 0, 65536), None,);
    assert_eq!(EpochNumberWithFraction::create(10000, 65536, 65535), None,);
    assert_eq!(EpochNumberWithFraction::create(10000, 1000, 1000), None,);
    assert_eq!(EpochNumberWithFraction::create(10000, 1001, 1000), None,);
    assert_eq!(
        EpochNumberWithFraction::create(16777215, 65534, 65535)
            .unwrap()
            .full_value(),
        0xFF_FFFF_FEFF_FFFF,
    );
    assert_eq!(
        EpochNumberWithFraction::create(1000, 1, 7).unwrap()
            + EpochNumberWithFraction::create(2000, 1, 5).unwrap(),
        Some(EpochNumberWithFraction::create(3000, 12, 35).unwrap()),
    );
    assert_eq!(
        EpochNumberWithFraction::create(100, 7, 13).unwrap()
            + EpochNumberWithFraction::create(50, 3, 5).unwrap(),
        Some(EpochNumberWithFraction::create(151, 9, 65).unwrap()),
    );
    assert_eq!(
        EpochNumberWithFraction::create(30, 3, 8).unwrap()
            + EpochNumberWithFraction::create(500, 5, 6).unwrap(),
        Some(EpochNumberWithFraction::create(531, 5, 24).unwrap()),
    );
    assert_eq!(
        EpochNumberWithFraction::create(1000, 1, 1001).unwrap()
            + EpochNumberWithFraction::create(2000, 7, 1003).unwrap(),
        None,
    );

    assert!(
        Since::from_block_number(1234, true).unwrap()
            < Since::from_block_number(2000, true).unwrap(),
    );
    assert!(
        Since::from_block_number(2001, false).unwrap()
            > Since::from_block_number(2000, false).unwrap(),
    );
    assert!(
        Since::from_timestamp(3111, true).unwrap() > Since::from_timestamp(2000, true).unwrap(),
    );
    assert!(
        Since::from_timestamp(1999, false).unwrap() < Since::from_timestamp(2000, false).unwrap(),
    );
    assert!(
        Since::from_epoch(
            EpochNumberWithFraction::create(100, 999, 1000).unwrap(),
            true
        ) < Since::from_epoch(EpochNumberWithFraction::create(101, 1, 1000).unwrap(), true),
    );
    assert!(
        Since::from_epoch(
            EpochNumberWithFraction::create(100, 600, 1000).unwrap(),
            true
        ) < Since::from_epoch(EpochNumberWithFraction::create(100, 8, 10).unwrap(), true),
    );
    assert_eq!(
        Since::from_block_number(1234, true)
            .unwrap()
            .partial_cmp(&Since::from_block_number(2000, false).unwrap()),
        None,
    );
    assert_eq!(
        Since::from_epoch(
            EpochNumberWithFraction::create(100, 999, 1000).unwrap(),
            false
        )
        .partial_cmp(&Since::from_epoch(
            EpochNumberWithFraction::create(101, 1, 1000).unwrap(),
            true
        )),
        None,
    );
    assert_eq!(
        Since::from_timestamp(1234, true)
            .unwrap()
            .partial_cmp(&Since::from_timestamp(2000, false).unwrap()),
        None,
    );
    assert_eq!(
        Since::from_block_number(1234, true)
            .unwrap()
            .partial_cmp(&Since::from_timestamp(2000, true).unwrap()),
        None,
    );
    assert_eq!(
        Since::from_block_number(1234, true)
            .unwrap()
            .partial_cmp(&Since::from_epoch(
                EpochNumberWithFraction::create(101, 1, 1000).unwrap(),
                true
            )),
        None,
    );
    assert_eq!(
        Since::from_timestamp(1234, true)
            .unwrap()
            .partial_cmp(&Since::from_epoch(
                EpochNumberWithFraction::create(101, 1, 1000).unwrap(),
                true
            )),
        None,
    );
}

#[cfg(target_arch = "riscv64")]
fn test_atomic() {
    // The bytes crate uses atomic operations.
    let b = bytes::Bytes::copy_from_slice(&[0, 1, 2, 3]);

    let b2 = b.slice(1..2);
    assert_eq!(b2[0], 1);
    assert_eq!(b2.len(), 1);

    let v: Vec<u8> = b.into();
    assert_eq!(v[1], 1);
    assert_eq!(v.len(), 4);

    // The log crate uses atomic operations.
    ckb_std::log::info!("atomic info");
    ckb_std::log::warn!("atomic warn");
}

#[cfg(target_arch = "riscv64")]
fn test_compare_exchange<T>(data: &mut T, expected: &mut T, desired: u64, same: bool) {
    let size = size_of_val(data);
    let res = match size {
        1 => dummy_atomic::__atomic_compare_exchange_1(
            data as *mut T as *mut c_void,
            expected as *mut T as *mut c_void,
            desired as u8,
            false,
            0,
            0,
        ),
        2 => dummy_atomic::__atomic_compare_exchange_2(
            data as *mut T as *mut c_void,
            expected as *mut T as *mut c_void,
            desired as u16,
            false,
            0,
            0,
        ),
        4 => dummy_atomic::__atomic_compare_exchange_4(
            data as *mut T as *mut c_void,
            expected as *mut T as *mut c_void,
            desired as u32,
            false,
            0,
            0,
        ),
        8 => dummy_atomic::__atomic_compare_exchange_8(
            data as *mut T as *mut c_void,
            expected as *mut T as *mut c_void,
            desired as u64,
            false,
            0,
            0,
        ),
        _ => {
            panic!("Unknown size");
        }
    };
    assert_eq!(res, same);
}

#[cfg(target_arch = "riscv64")]
fn test_atomic2() {
    let mut data1: u8 = 42;
    let old = dummy_atomic::__atomic_exchange_1(&mut data1 as *mut u8 as *mut c_void, 0, 0);
    assert_eq!(old, 42);
    let mut data2: u16 = 42;
    let old = dummy_atomic::__atomic_exchange_2(&mut data2 as *mut u16 as *mut c_void, 0, 0);
    assert_eq!(old, 42);
    let mut data4: u32 = 42;
    let old = dummy_atomic::__atomic_exchange_4(&mut data4 as *mut u32 as *mut c_void, 0, 0);
    assert_eq!(old, 42);
    let mut data8: u64 = 42;
    let old = dummy_atomic::__atomic_exchange_8(&mut data8 as *mut u64 as *mut c_void, 0, 0);
    assert_eq!(old, 42);

    let mut data: u8 = 42;
    let mut expected: u8 = 42;
    test_compare_exchange(&mut data, &mut expected, 0, true);
    assert_eq!(data, 0);
    test_compare_exchange(&mut data, &mut expected, 0, false);
    assert_eq!(expected, 0);

    let mut data: u16 = 42;
    let mut expected: u16 = 42;
    test_compare_exchange(&mut data, &mut expected, 0, true);
    assert_eq!(data, 0);
    test_compare_exchange(&mut data, &mut expected, 0, false);
    assert_eq!(expected, 0);

    let mut data: u32 = 42;
    let mut expected: u32 = 42;
    test_compare_exchange(&mut data, &mut expected, 0, true);
    assert_eq!(data, 0);
    test_compare_exchange(&mut data, &mut expected, 0, false);
    assert_eq!(expected, 0);

    let mut data: u64 = 42;
    let mut expected: u64 = 42;
    test_compare_exchange(&mut data, &mut expected, 0, true);
    assert_eq!(data, 0);
    test_compare_exchange(&mut data, &mut expected, 0, false);
    assert_eq!(expected, 0);

    let data: u8 = 42;
    let expected = dummy_atomic::__atomic_load_1(&data as *const u8 as *const c_void, 0);
    assert_eq!(expected, data);

    let data: u16 = 42;
    let expected = dummy_atomic::__atomic_load_2(&data as *const u16 as *const c_void, 0);
    assert_eq!(expected, data);

    let data: u32 = 42;
    let expected = dummy_atomic::__atomic_load_4(&data as *const u32 as *const c_void, 0);
    assert_eq!(expected, data);

    let data: u64 = 42;
    let expected = dummy_atomic::__atomic_load_8(&data as *const u64 as *const c_void, 0);
    assert_eq!(expected, data);

    let mut data: u8 = 42;
    dummy_atomic::__atomic_store_1(&mut data as *mut u8 as *mut c_void, 0, 0);
    assert_eq!(data, 0);

    let mut data: u16 = 42;
    dummy_atomic::__atomic_store_2(&mut data as *mut u16 as *mut c_void, 0, 0);
    assert_eq!(data, 0);

    let mut data: u32 = 42;
    dummy_atomic::__atomic_store_4(&mut data as *mut u32 as *mut c_void, 0, 0);
    assert_eq!(data, 0);

    let mut data: u64 = 42;
    dummy_atomic::__atomic_store_8(&mut data as *mut u64 as *mut c_void, 0, 0);
    assert_eq!(data, 0);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_fetch_add_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_fetch_add_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_fetch_add_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_fetch_add_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_fetch_sub_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 41);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_fetch_sub_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 41);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_fetch_sub_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 41);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_fetch_sub_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 41);

    let mut data: u8 = 43;
    let res = dummy_atomic::__atomic_fetch_and_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 1);

    let mut data: u16 = 43;
    let res = dummy_atomic::__atomic_fetch_and_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 1);

    let mut data: u32 = 43;
    let res = dummy_atomic::__atomic_fetch_and_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 1);

    let mut data: u64 = 43;
    let res = dummy_atomic::__atomic_fetch_and_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 1);

    let mut data: u8 = 43;
    let res = dummy_atomic::__atomic_fetch_xor_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 42);

    let mut data: u16 = 43;
    let res = dummy_atomic::__atomic_fetch_xor_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 42);

    let mut data: u32 = 43;
    let res = dummy_atomic::__atomic_fetch_xor_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 42);

    let mut data: u64 = 43;
    let res = dummy_atomic::__atomic_fetch_xor_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 42);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_fetch_or_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_fetch_or_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_fetch_or_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_fetch_or_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 43);

    let mut data: u8 = 1;
    let res = dummy_atomic::__atomic_fetch_nand_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 0xFE);

    let mut data: u16 = 1;
    let res = dummy_atomic::__atomic_fetch_nand_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 0xFFFE);

    let mut data: u32 = 1;
    let res = dummy_atomic::__atomic_fetch_nand_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 0xFFFFFFFE);

    let mut data: u64 = 1;
    let res = dummy_atomic::__atomic_fetch_nand_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 0xFFFFFFFFFFFFFFFE);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_add_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_add_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_add_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_add_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_sub_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 41);
    assert_eq!(data, 41);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_sub_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 41);
    assert_eq!(data, 41);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_sub_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 41);
    assert_eq!(data, 41);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_sub_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 41);
    assert_eq!(data, 41);

    let mut data: u8 = 43;
    let res = dummy_atomic::__atomic_and_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 1);

    let mut data: u16 = 43;
    let res = dummy_atomic::__atomic_and_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 1);

    let mut data: u32 = 43;
    let res = dummy_atomic::__atomic_and_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 1);

    let mut data: u64 = 43;
    let res = dummy_atomic::__atomic_and_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 1);
    assert_eq!(data, 1);

    let mut data: u8 = 43;
    let res = dummy_atomic::__atomic_xor_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 42);

    let mut data: u16 = 43;
    let res = dummy_atomic::__atomic_xor_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 42);

    let mut data: u32 = 43;
    let res = dummy_atomic::__atomic_xor_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 42);

    let mut data: u64 = 43;
    let res = dummy_atomic::__atomic_xor_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 42);
    assert_eq!(data, 42);

    let mut data: u8 = 42;
    let res = dummy_atomic::__atomic_or_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u16 = 42;
    let res = dummy_atomic::__atomic_or_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u32 = 42;
    let res = dummy_atomic::__atomic_or_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u64 = 42;
    let res = dummy_atomic::__atomic_or_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 43);
    assert_eq!(data, 43);

    let mut data: u8 = 1;
    let res = dummy_atomic::__atomic_nand_fetch_1(&mut data as *mut u8 as *mut c_void, 1, 0);
    assert_eq!(res, 0xFE);
    assert_eq!(data, 0xFE);

    let mut data: u16 = 1;
    let res = dummy_atomic::__atomic_nand_fetch_2(&mut data as *mut u16 as *mut c_void, 1, 0);
    assert_eq!(res, 0xFFFE);
    assert_eq!(data, 0xFFFE);

    let mut data: u32 = 1;
    let res = dummy_atomic::__atomic_nand_fetch_4(&mut data as *mut u32 as *mut c_void, 1, 0);
    assert_eq!(res, 0xFFFFFFFE);
    assert_eq!(data, 0xFFFFFFFE);

    let mut data: u64 = 1;
    let res = dummy_atomic::__atomic_nand_fetch_8(&mut data as *mut u64 as *mut c_void, 1, 0);
    assert_eq!(res, 0xFFFFFFFFFFFFFFFE);
    assert_eq!(data, 0xFFFFFFFFFFFFFFFE);
}

#[cfg(target_arch = "riscv64")]
fn test_log() {
    drop(ckb_std::logger::init());
    ckb_std::log::trace!("this is trace");
    ckb_std::log::debug!("this is debug");
    ckb_std::log::info!("this is info");
    ckb_std::log::warn!("this is warn");
    ckb_std::log::error!("this is error");
}

#[cfg(target_arch = "riscv64")]
lazy_static! {
    // Context should not be dropped.
    static ref old_context: Mutex<ContextTypeOld> = {
        #[allow(deprecated)]
        Mutex::new(unsafe { ContextTypeOld::new() })
    };
    // Context should not be dropped.
    static ref new_context: Mutex<ContextType> = {
        Mutex::new(unsafe { ContextType::new() })
    };
}

pub fn main() -> Result<(), Error> {
    test_basic();
    test_load_data();
    test_load_cell_field();
    test_load_tx_hash();
    test_partial_load_tx_hash();
    test_high_level_apis();
    test_query();
    test_calc_data_hash();

    #[cfg(target_arch = "riscv64")]
    test_dynamic_loading(&mut old_context.lock());
    #[cfg(target_arch = "riscv64")]
    test_dynamic_loading_c_impl(&mut new_context.lock());

    test_vm_version();
    test_current_cycles();
    test_since();
    #[cfg(target_arch = "riscv64")]
    {
        test_atomic();
        test_atomic2();
        test_log();
    }

    Ok(())
}
