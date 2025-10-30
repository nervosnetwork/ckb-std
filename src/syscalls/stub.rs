//! This module implements syscalls in old ckb-std style based on
//! an impl of SyscallImpls trait.

use crate::{
    ckb_constants::{CellField, HeaderField, InputField, Place, Source},
    error::SysError,
    syscalls::traits::{Bounds, SyscallImpls},
};
use alloc::{boxed::Box, string::String, vec::Vec};
use core::ffi::CStr;

pub use crate::syscalls::internal::SpawnArgs;

// We cannot use OnceCell here: some fuzzer engines(such as AFL++) reuse the
// same process to run multiple iterations of fuzzed code. For each iteration,
// a new object implementing SyscallImpls is likely required. In this case,
// current value needs to be updated multiple times, which will not be possible
// with OnceCell.
static mut IMPLS: Option<Box<dyn SyscallImpls>> = None;

/// Initializes a new SyscallImpls trait impl.
pub fn init(impls: Box<dyn SyscallImpls>) {
    unsafe { IMPLS = Some(impls) }
}

/// # Safety
///
/// This shall be safe since CKB-VM uses a single threaded environment.
fn get() -> &'static Box<dyn SyscallImpls> {
    // https://doc.rust-lang.org/nightly/edition-guide/rust-2024/static-mut-references.html#no_std-one-time-initialization
    let Some(impls) = (unsafe { &*&raw const IMPLS }) else {
        panic!("No IMPLS provided!");
    };
    impls
}

pub fn close(fd: u64) -> Result<(), SysError> {
    get().close(fd)?;
    Ok(())
}

pub fn current_cycles() -> u64 {
    get().current_cycles()
}

pub fn debug(s: String) {
    get().debug_s(s)
}

pub fn exec(index: usize, source: Source, place: usize, bounds: usize, argv: &[&CStr]) -> u64 {
    let result = get().exec(
        index,
        source,
        Place::try_from(place as u64).unwrap(),
        Bounds::from(bounds as u64),
        argv,
    );
    match result {
        Ok(_) => 0,
        Err(e) => e.into(),
    }
}

pub fn exit(code: i8) -> ! {
    get().exit(code)
}

pub fn inherited_fds(fds: &mut [u64]) -> u64 {
    get().inherited_fds(fds).unwrap() as u64
}

pub fn load_block_extension(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get()
        .load_block_extension(buf, offset, index, source)
        .into()
}

pub fn load_cell(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get().load_cell(buf, offset, index, source).into()
}

pub fn load_cell_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: CellField,
) -> Result<usize, SysError> {
    get()
        .load_cell_by_field(buf, offset, index, source, field)
        .into()
}

pub fn load_cell_code(
    buf_ptr: *mut u8,
    len: usize,
    content_offset: usize,
    content_size: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let result = get().load_cell_code(buf_ptr, len, content_offset, content_size, index, source);
    match result {
        Ok(()) => Ok(len),
        Err(e) => Err(e.into()),
    }
}

pub fn load_cell_data(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get().load_cell_data(buf, offset, index, source).into()
}

pub fn load_cell_data_raw(
    buf_ptr: *mut u8,
    len: usize,
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    let buf = unsafe { core::slice::from_raw_parts_mut(buf_ptr, len) };

    get().load_cell_data(buf, offset, index, source).into()
}

pub fn load_header(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get().load_header(buf, offset, index, source).into()
}

pub fn load_header_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: HeaderField,
) -> Result<usize, SysError> {
    get()
        .load_header_by_field(buf, offset, index, source, field)
        .into()
}

pub fn load_input(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get().load_input(buf, offset, index, source).into()
}

pub fn load_input_by_field(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
    field: InputField,
) -> Result<usize, SysError> {
    get()
        .load_input_by_field(buf, offset, index, source, field)
        .into()
}

pub fn load_script(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    get().load_script(buf, offset).into()
}

pub fn load_script_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    get().load_script_hash(buf, offset).into()
}

pub fn load_transaction(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    get().load_transaction(buf, offset).into()
}

pub fn load_tx_hash(buf: &mut [u8], offset: usize) -> Result<usize, SysError> {
    get().load_tx_hash(buf, offset).into()
}

pub fn load_witness(
    buf: &mut [u8],
    offset: usize,
    index: usize,
    source: Source,
) -> Result<usize, SysError> {
    get().load_witness(buf, offset, index, source).into()
}

pub fn pipe() -> Result<(u64, u64), SysError> {
    let pipes = get().pipe()?;
    Ok(pipes)
}

pub fn process_id() -> u64 {
    get().process_id()
}

pub fn read(fd: u64, buffer: &mut [u8]) -> Result<usize, SysError> {
    let read = get().read(fd, buffer)?;
    Ok(read)
}

pub fn spawn(
    index: usize,
    source: Source,
    place: usize,
    bounds: usize,
    spgs: &mut SpawnArgs,
) -> Result<u64, SysError> {
    let mut argv = Vec::with_capacity(spgs.argc as usize);
    for i in 0..spgs.argc {
        let p = unsafe { spgs.argv.offset(i as isize).read() };
        argv.push(unsafe { CStr::from_ptr(p as *const _) });
    }
    let mut fds = Vec::new();
    {
        let mut i = 0;
        loop {
            let fd = unsafe { spgs.inherited_fds.offset(i).read() };
            if fd == 0 {
                break;
            }
            fds.push(fd);
            i += 1;
        }
    }
    let process_id = get().spawn(
        index,
        source,
        Place::try_from(place as u64).unwrap(),
        Bounds::from(bounds as u64),
        &argv,
        &fds,
    )?;
    unsafe { spgs.process_id.write(process_id) }
    Ok(process_id)
}

pub fn vm_version() -> Result<u64, SysError> {
    let version = get().vm_version();
    // Personally I think this logic does not make sense but we will just keep
    // ckb-std's convention.
    match version {
        1 | 2 => Ok(version),
        _ => Err(SysError::Unknown(version)),
    }
}

pub fn wait(pid: u64) -> Result<i8, SysError> {
    let exit_code = get().wait(pid)?;
    Ok(exit_code)
}

pub fn write(fd: u64, buffer: &[u8]) -> Result<usize, SysError> {
    let written = get().write(fd, buffer)?;
    Ok(written)
}

// SAFETY: this is hidden under a feature, and is only designed to
// override existing function of the same name.
#[cfg(feature = "stub-c-syscalls")]
#[unsafe(no_mangle)]
pub extern "C" fn __internal_syscall(
    n: core::ffi::c_long,
    a0: core::ffi::c_long,
    a1: core::ffi::c_long,
    a2: core::ffi::c_long,
    a3: core::ffi::c_long,
    a4: core::ffi::c_long,
    a5: core::ffi::c_long,
) -> core::ffi::c_long {
    crate::syscalls::traits::syscall_to_impls(
        &**get(),
        n as u64,
        a0 as u64,
        a1 as u64,
        a2 as u64,
        a3 as u64,
        a4 as u64,
        a5 as u64,
    ) as core::ffi::c_long
}
