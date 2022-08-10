extern crate alloc;

#[path = "../../../contracts/ckb-std-tests/src/entry.rs"]
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

fn main() {
    let code = entry::main()
        .map(|()| 0i32)
        .unwrap_or_else(|err| err as i32);
    if code != 0 {
        println!("exit with {}", code);
    }
    std::process::exit(code);
}
