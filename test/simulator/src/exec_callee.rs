extern crate alloc;

#[path = "../../../contracts/exec-callee/src/entry.rs"]
mod entry;

pub mod error {
    use ckb_std::error::SysError;
    /// Error
    #[repr(i8)]
    pub enum Error {
        IndexOutOfBound = 1,
        ItemMissing,
        LengthNotEnough,
        Encoding,
        // Add customized errors here...
    }

    impl From<SysError> for Error {
        fn from(err: SysError) -> Self {
            use SysError::*;
            match err {
                IndexOutOfBound => Self::IndexOutOfBound,
                ItemMissing => Self::ItemMissing,
                LengthNotEnough(_) => Self::LengthNotEnough,
                Encoding => Self::Encoding,
                Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            }
        }
    }
}

use std::env;
use std::ffi::CString;
use std::os::unix::ffi::OsStringExt;

fn main() {
    println!("START simulator callee");
    let args = env::args_os()
        .into_iter()
        .map(|arg| CString::new(arg.into_vec()).unwrap())
        .collect::<Vec<_>>();
    let argc = args.len() as u64;
    let mut argv = args
        .iter()
        .map(|cstring| cstring.to_bytes_with_nul().as_ptr())
        .collect::<Vec<_>>();
    argv.push(std::ptr::null());
    println!("START simulator callee entry");
    let code = entry::main(argc, argv.as_ptr())
        .map(|()| 0i32)
        .unwrap_or_else(|err| err as i32);
    if code != 0 {
        println!("exit with {}", code);
    } else {
        println!("simulator callee run success");
    }
    std::process::exit(code);
}
