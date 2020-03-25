#[macro_export]
macro_rules! debug {
    ($fmt:literal) => {
        $crate::syscalls::debug(alloc::format!($fmt));
    };
    ($fmt:literal, $($args:expr),+) => {
        $crate::syscalls::debug(alloc::format!($fmt, $($args), +));
    };
}
