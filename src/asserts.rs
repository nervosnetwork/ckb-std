use core::fmt::Debug;

pub static mut __PANIC_EXIT_CODE: i8 = -1;

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
    }};
}

#[macro_export]
macro_rules! set_error_assert_eq {
    ($code:expr, $($arg:tt)*) => {{
        $crate::asserts::set_panic_exit_code($code);
        assert_eq!($($arg)*);
    }};
}

pub fn set_error_expect<C: Into<i8>, T, E: Debug>(c: C, r: Result<T, E>, msg: &str) -> T {
    set_panic_exit_code(c.into());
    r.expect(msg)
}
