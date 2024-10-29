/// debug macro
///
/// Output a debug message.
///
/// This macro only compiled under debug build and does nothing in release build. To debug the release build,
/// include `--cfg debug_assertions` in the environment variable `RUSTFLAGS` before calling `cargo build`.
/// For example:
///
/// ```
/// RUSTFLAGS="--cfg debug_assertions" cargo build --release --target=riscv64imac-unknown-none-elf
/// ```
///
/// For users of Capsule, the debug macro can be enabled in the release build by running
/// `capsule build --release --debug-output`.
///
/// Notice: to see the debug output, you must turn on `ckb_script` debugging log in the CKB node configuration
/// like this:
///
/// ```toml
/// [logger]
/// filter = "info,ckb-script=debug"
/// ```
///
/// See the essay on more [Tips for Debugging CKB Scripts](https://docs.nervos.org/docs/essays/debug).
///
/// # Example
///
/// ```
/// debug!("hello world");
/// debug!("there is a universal error caused by {}", 42);
/// ```
#[macro_export]
macro_rules! debug {
    ($fmt:literal) => {
        #[cfg(debug_assertions)]
        $crate::syscalls::debug(format!($fmt));
    };
    ($fmt:literal, $($args:expr),+) => {
        #[cfg(debug_assertions)]
        $crate::syscalls::debug(format!($fmt, $($args), +));
    };
}
