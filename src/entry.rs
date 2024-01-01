/// Define program entry point (`_start` function) and lang items (panic handler, etc.).
///
/// # Examples
///
/// Simple main function:
///
/// ```
/// entry!(main)
///
/// fn main() -> i8 {
///    0
/// }
/// ```
#[macro_export]
macro_rules! entry {
    ($main:path) => {
        extern crate alloc;

        #[cfg(not(target_arch = "riscv64"))]
        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            panic!("ckb_std::entry is only valid for riscv64 target")
        }

        #[no_mangle]
        unsafe extern "C" fn __ckb_std_main(
            argc: core::ffi::c_int,
            // Arg is the same as *const c_char ABI wise.
            argv: *const $crate::env::Arg,
        ) -> i8 {
            let argv = core::slice::from_raw_parts(argv, argc as usize);
            $crate::env::set_argv(argv);
            $main()
        }

        // Use global_asm so the compiler won't insert function prologue in _start.
        #[cfg(target_arch = "riscv64")]
        core::arch::global_asm!(
            ".global _start",
            "_start:",
            // Argc.
            "lw a0, 0(sp)",
            // Argv.
            "addi a1, sp, 8",
            // Envp.
            "li a2, 0",
            "call __ckb_std_main",
            // Exit.
            "li a7, 93",
            "ecall",
        );

        #[cfg(target_arch = "riscv64")]
        #[panic_handler]
        fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
            $crate::debug!("{}", panic_info);
            $crate::syscalls::exit(-1)
        }
    };
}
