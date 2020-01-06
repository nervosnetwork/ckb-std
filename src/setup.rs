#[macro_export]
macro_rules! setup {
    ($main:path) => {
        extern crate alloc;

        pub static mut _HEAP: [u8; ckb_contract_std::buddy_alloc::REQUIRED_SPACE] =
            [0u8; ckb_contract_std::buddy_alloc::REQUIRED_SPACE];

        #[global_allocator]
        static ALLOC: ckb_contract_std::buddy_alloc::NonThreadsafeAlloc =
            unsafe { ckb_contract_std::buddy_alloc::NonThreadsafeAlloc::new(_HEAP.as_ptr()) };

        #[alloc_error_handler]
        fn oom_handler(layout: alloc::alloc::Layout) -> ! {
            panic!("allocate memory error {:?}", layout)
        }

        #[no_mangle]
        pub extern "C" fn _start() -> ! {
            let f: fn() -> i8 = $main;
            ckb_contract_std::syscalls::exit(f())
        }

        #[lang = "eh_personality"]
        extern "C" fn eh_personality() {}

        /// Fix symbol missing
        #[no_mangle]
        pub extern "C" fn abort() {
            panic!("abort!");
        }

        #[panic_handler]
        fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
            use alloc::format;

            let mut s = alloc::string::String::new();
            if let Some(p) = panic_info.payload().downcast_ref::<&str>() {
                s.push_str(&format!("panic occurred: {:?}", p));
            } else {
                s.push_str(&format!("panic occurred:"));
            }
            if let Some(m) = panic_info.message() {
                s.push_str(&format!(" {:?}", m));
            }
            if let Some(location) = panic_info.location() {
                s.push_str(&format!(
                    ", in file {}:{}",
                    location.file(),
                    location.line()
                ));
            } else {
                s.push_str(&format!(", but can't get location information..."));
            }

            ckb_contract_std::syscalls::debug(s);
            ckb_contract_std::syscalls::exit(-1)
        }
    };
}
