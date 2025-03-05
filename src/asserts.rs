use core::fmt::Debug;

pub const DEFAULT_PANIC_EXIT_CODE: i8 = -1;

pub static mut __PANIC_EXIT_CODE: i8 = DEFAULT_PANIC_EXIT_CODE;

pub fn set_panic_exit_code(code: i8) {
    unsafe {
        __PANIC_EXIT_CODE = code;
    }
}

#[macro_export]
macro_rules! set_error_assert {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        assert!($($arg)*);
        $crate::asserts::set_panic_exit_code($crate::asserts::DEFAULT_PANIC_EXIT_CODE);
    }};
}

#[macro_export]
macro_rules! set_error_assert_eq {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        assert_eq!($($arg)*);
        $crate::asserts::set_panic_exit_code($crate::asserts::DEFAULT_PANIC_EXIT_CODE);
    }};
}

pub fn set_error_expect<C: Into<i8>, T, E: Debug>(c: C, r: Result<T, E>, msg: &str) -> T {
    set_panic_exit_code(c.into());
    let t = r.expect(msg);
    set_panic_exit_code(DEFAULT_PANIC_EXIT_CODE);
    t
}
