use core::fmt::Debug;

pub const DEFAULT_PANIC_EXIT_CODE: i8 = -1;

pub static mut __PANIC_EXIT_CODE: i8 = DEFAULT_PANIC_EXIT_CODE;

pub fn set_panic_exit_code(code: i8) {
    unsafe {
        __PANIC_EXIT_CODE = code;
    }
}

#[macro_export]
macro_rules! assert {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        core::assert!($($arg)*);
        $crate::asserts::set_panic_exit_code($crate::asserts::DEFAULT_PANIC_EXIT_CODE);
    }};
}

#[macro_export]
macro_rules! assert_eq {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        core::assert_eq!($($arg)*);
        $crate::asserts::set_panic_exit_code($crate::asserts::DEFAULT_PANIC_EXIT_CODE);
    }};
}

#[macro_export]
macro_rules! assert_ne {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        core::assert_ne!($($arg)*);
        $crate::asserts::set_panic_exit_code($crate::asserts::DEFAULT_PANIC_EXIT_CODE);
    }};
}

pub fn expect_result<C: Into<i8>, T, E: Debug>(c: C, r: Result<T, E>, msg: &str) -> T {
    set_panic_exit_code(c.into());
    let t = r.expect(msg);
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    t
}

pub fn expect_err_result<C: Into<i8>, T: Debug, E>(c: C, r: Result<T, E>, msg: &str) -> E {
    set_panic_exit_code(c.into());
    let e = r.expect_err(msg);
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    e
}

pub fn unwrap_result<C: Into<i8>, T, E: Debug>(c: C, r: Result<T, E>) -> T {
    set_panic_exit_code(c.into());
    let t = r.unwrap();
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    t
}

pub fn unwrap_err_result<C: Into<i8>, T: Debug, E>(c: C, r: Result<T, E>) -> E {
    set_panic_exit_code(c.into());
    let e = r.unwrap_err();
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    e
}

pub fn expect_option<C: Into<i8>, T>(c: C, o: Option<T>, msg: &str) -> T {
    set_panic_exit_code(c.into());
    let t = o.expect(msg);
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    t
}

pub fn unwrap_option<C: Into<i8>, T>(c: C, o: Option<T>) -> T {
    set_panic_exit_code(c.into());
    let t = o.unwrap();
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    t
}
