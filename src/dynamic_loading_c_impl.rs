use crate::debug;
use crate::error::SysError;
use ckb_types::core::ScriptHashType;
use core::ffi::c_void;
use core::marker::PhantomData;
use core::mem::{size_of, zeroed};
use core::ptr::null;

#[link(name = "dl-c-impl", kind = "static")]
extern "C" {
    fn ckb_dlopen2(
        dep_cell_hash: *const u8,
        hash_type: u8,
        aligned_addr: *mut u8,
        aligned_size: usize,
        handle: *mut *const c_void,
        consumed_size: *mut usize,
    ) -> isize;
    fn ckb_dlsym(handle: *const c_void, symbol: *const u8) -> usize;
}

/// Dynamic loading errors
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Create context error
    ContextFailure,
    /// Invalid alignment
    InvalidAlign,
    /// Syscall error
    Sys(SysError),
    /// ckb_dlopen2 failed
    OpenFailed(isize),
    /// Currently, only Data and Data1 are valid in load_by_data
    InvalidHashTypeData,
}

impl From<SysError> for Error {
    fn from(error: SysError) -> Error {
        Error::Sys(error)
    }
}

/// Wrapper of dynamic loaded symbols
pub struct Symbol<T> {
    ptr: usize,
    phantom: PhantomData<T>,
}

impl<T> Symbol<T> {
    fn new(ptr: usize) -> Self {
        Symbol {
            ptr,
            phantom: PhantomData,
        }
    }
}

impl<T> core::ops::Deref for Symbol<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(&self.ptr) }
    }
}

/// Dynamic loaded library
pub struct Library {
    handle: *const c_void,
    consumed_size: usize,
}

impl Library {
    fn new() -> Self {
        Library {
            handle: null(),
            consumed_size: 0,
        }
    }

    /// Library consumed size
    pub fn consumed_size(&self) -> usize {
        self.consumed_size
    }

    /// # Unsafe
    ///
    /// Undefined behavior will happen if the type S not match the type of symbol in the shared
    /// library
    ///
    /// Return None if not found the symbol
    pub unsafe fn get<S>(&self, symbol: &[u8]) -> Option<Symbol<S>> {
        let mut s = symbol.to_vec();
        if s.len() > 0 {
            if s[s.len() - 1] != 0 {
                s.push(0);
            }
        } else {
            panic!("symbol with zero length");
        }
        let ptr = ckb_dlsym(self.handle, s.as_ptr());
        if ptr == 0 {
            debug!(
                "warning, ckb_dlsym returns 0, handle = {:?}, symbol = {:?}",
                self.handle, symbol
            );
            None
        } else {
            Some(Symbol::new(ptr))
        }
    }
}

const RISCV_PGSIZE_SHIFT: usize = 12;
const RISCV_PGSIZE: usize = 1 << RISCV_PGSIZE_SHIFT; // 4096

#[repr(C)]
#[repr(align(4096))]
pub struct CKBDLContext<T>(T);

impl<T> CKBDLContext<T> {
    pub unsafe fn new() -> Self {
        zeroed()
    }
    pub fn load_with_offset<'a>(
        &'a mut self,
        dep_cell_data_hash: &[u8],
        offset: usize,
        size: usize,
        hash_type: ScriptHashType,
    ) -> Result<Library, Error> {
        if size_of::<Library>() > RISCV_PGSIZE || size < RISCV_PGSIZE {
            return Err(Error::ContextFailure);
        }
        // size must aligned to page size
        if ((size >> RISCV_PGSIZE_SHIFT) << RISCV_PGSIZE_SHIFT) != size {
            return Err(Error::InvalidAlign);
        }
        let from_type_id = match hash_type {
            ScriptHashType::Type => 1,
            _ => 0,
        };
        unsafe {
            let mut handle: *const c_void = null();
            let mut consumed_size: usize = 0;
            let mut library = Library::new();
            let aligned_size = size;
            let aligned_addr = (&mut self.0 as *mut T).cast::<u8>().add(offset);
            let code = ckb_dlopen2(
                dep_cell_data_hash.as_ptr(),
                from_type_id,
                aligned_addr,
                aligned_size,
                &mut handle as *mut *const c_void,
                &mut consumed_size as *mut usize,
            );
            if code != 0 {
                debug!("warning, ckb_dlopen2 return {:?}", code);
                return Err(Error::OpenFailed(code));
            } else {
                library.handle = handle;
                library.consumed_size = consumed_size;
                return Ok(library);
            }
        }
    }
    #[deprecated(
        since = "0.10.0",
        note = "Please use load_by_data or load_by_type_id instead"
    )]
    pub fn load<'a>(&'a mut self, dep_cell_data_hash: &[u8]) -> Result<Library, Error> {
        self.load_by_data(dep_cell_data_hash, ScriptHashType::Data)
    }
    ///
    /// load library by data hash
    ///
    pub fn load_by_data<'a>(
        &'a mut self,
        dep_cell_data_hash: &[u8],
        hash_type: ScriptHashType,
    ) -> Result<Library, Error> {
        // currently, we only have Data and Data1 with same logic.
        // In the future hardfork, we might add more types and restrictions.
        if hash_type != ScriptHashType::Data && hash_type != ScriptHashType::Data1 {
            return Err(Error::InvalidHashTypeData);
        }
        self.load_with_offset(
            dep_cell_data_hash,
            0,
            size_of::<CKBDLContext<T>>(),
            hash_type,
        )
    }
    ///
    /// load library by type id
    ///
    pub fn load_by_type_id<'a>(&'a mut self, type_id: &[u8]) -> Result<Library, Error> {
        self.load_with_offset(
            type_id,
            0,
            size_of::<CKBDLContext<T>>(),
            ScriptHashType::Type,
        )
    }
}
