ckb_std::entry_simulator!(program_entry);

mod exec_callee {
    use core::result::Result;

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
    }

    impl From<ckb_std::error::SysError> for Error {
        fn from(err: ckb_std::error::SysError) -> Self {
            match err {
                ckb_std::error::SysError::IndexOutOfBound => Self::IndexOutOfBound,
                ckb_std::error::SysError::ItemMissing => Self::ItemMissing,
                ckb_std::error::SysError::LengthNotEnough(_) => Self::LengthNotEnough,
                ckb_std::error::SysError::Encoding => Self::Encoding,
                ckb_std::error::SysError::WaitFailure => Self::WaitFailure,
                ckb_std::error::SysError::InvalidFd => Self::InvalidFd,
                ckb_std::error::SysError::OtherEndClosed => Self::OtherEndClosed,
                ckb_std::error::SysError::MaxVmsSpawned => Self::MaxVmsSpawned,
                ckb_std::error::SysError::MaxFdsCreated => Self::MaxFdsCreated,
                ckb_std::error::SysError::Unknown(err_code) => {
                    panic!("unexpected sys error {}", err_code)
                }
            }
        }
    }

    pub fn main() -> Result<(), Error> {
        let argv = ckb_std::env::argv();
        ckb_std::debug!("argv: {:?}", argv);
        assert_eq!(argv.len(), 2);
        assert_eq!(argv[0].to_bytes(), b"Hello World");
        assert_eq!(argv[1].to_bytes(), "你好".as_bytes());
        Ok(())
    }
}

/// program entry
fn program_entry() -> i8 {
    // Call main function and return error code
    match exec_callee::main() {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
