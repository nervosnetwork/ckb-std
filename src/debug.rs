#[macro_export]
macro_rules! debug {
    ($fmt:literal) => {
        ckb_contract_std::syscalls::debug(alloc::format!($fmt));
    };
    ($fmt:literal, $($args:expr),+) => {
        ckb_contract_std::syscalls::debug(alloc::format!($fmt, $($args), +));
    };
}
