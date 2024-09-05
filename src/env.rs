//! Inspection and manipulation of the program’s environment.

use core::{
    ffi::{c_char, CStr},
    fmt::Debug,
    ops::Deref,
};

/// An argument passed to this program.
#[repr(transparent)]
pub struct Arg(*const c_char);

impl Debug for Arg {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.deref().fmt(f)
    }
}

impl From<&'static CStr> for Arg {
    #[inline]
    fn from(arg: &'static CStr) -> Self {
        Self(arg.as_ptr())
    }
}

impl Deref for Arg {
    type Target = CStr;
    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { CStr::from_ptr(self.0) }
    }
}

#[cfg(feature = "native-simulator")]
impl Arg {
    pub fn new(arg: &str) -> Self {
        Self {
            0: (arg.as_ptr()) as *const c_char,
        }
    }
}

static mut ARGV: &'static [Arg] = &[];

/// Returns the arguments that this program was started with (normally passed
/// via `exec` or ckb-debugger).
///
/// (Not to be confused with **script args** when used as cell lock script or
/// type script. That has be loaded with `load_script`.)
#[inline]
pub fn argv() -> &'static [Arg] {
    unsafe { ARGV }
}

// For native-simulator and entry!.
#[doc(hidden)]
#[inline]
pub unsafe fn set_argv(argv: &'static [Arg]) {
    ARGV = argv
}
