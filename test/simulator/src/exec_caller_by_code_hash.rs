extern crate alloc;

mod entry {
    use core::ffi::CStr;
    use ckb_std::{
        ckb_types::core::ScriptHashType,
        high_level::{self, load_script},
    };

    use crate::error::Error;

    pub fn main() -> Result<(), Error> {
        let arg1 = CStr::from_bytes_with_nul(b"Hello World\0").unwrap();
        let arg2 = CStr::from_bytes_with_nul("你好\0".as_bytes()).unwrap();
        let code_hash = load_script().unwrap().args().raw_data();
        ckb_std::debug!("code_hash: {:?}", code_hash);
        high_level::exec_cell(&code_hash[..], ScriptHashType::Data1, &[arg1, arg2][..])?;
        unreachable!()
    }
}

pub mod error {
    use ckb_std::error::SysError;
    /// Error
    #[repr(i8)]
    pub enum Error {
        IndexOutOfBound = 1,
        ItemMissing,
        LengthNotEnough,
        Encoding,
        WaitFailure,
        InvalidFd,
        OtherEndClosed,
        MaxVmsSpawned,
        MaxFdsCreated,
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
                WaitFailure => Self::WaitFailure,
                InvalidFd => Self::InvalidFd,
                OtherEndClosed => Self::OtherEndClosed,
                MaxVmsSpawned => Self::MaxVmsSpawned,
                MaxFdsCreated => Self::MaxFdsCreated,
                Unknown(err_code) => panic!("unexpected sys error {}", err_code),
            }
        }
    }
}

fn main() {
    let code = entry::main()
        .map(|()| 0i32)
        .unwrap_or_else(|err| err as i32);
    if code != 0 {
        println!("exit with {}", code);
    }
    std::process::exit(code);
}
