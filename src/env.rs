//! Inspection and manipulation of the program’s environment.

use core::ffi::CStr;

static mut ARGV: &'static [&'static CStr] = &[];

/// Returns the arguments that this program was started with (normally passed
/// via `exec` or ckb-debugger).
///
/// (Not to be confused with **script args** when used as cell lock script or
/// type script. That has be loaded with `load_script`.)
pub fn argv() -> &'static [&'static CStr] {
    unsafe { ARGV }
}

// For simulator and entry!.
#[doc(hidden)]
pub unsafe fn set_argv(argv: &'static [&'static CStr]) {
    ARGV = argv
}
