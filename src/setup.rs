/// Usage
///
/// ``` rust
/// // define an entry point and initialize ckb-contract-std runtime
/// setup!(main)
/// // to indicate the heap size(default heap size is 1MB)
/// setup!(main, 2 * 1024 * 1024)
/// ```
#[macro_export]
macro_rules! setup {
    ($main:path) => {
        setup!($main, 1024 * 1024);
    };
    ($main:path, $heap_size:expr) => {
        extern crate alloc;
        const _HEAP_SIZE: usize = $heap_size;

        static mut _HEAP: [u8; _HEAP_SIZE] = [0u8; _HEAP_SIZE];

        #[global_allocator]
        static ALLOC: ckb_contract_std::buddy_alloc::NonThreadsafeAlloc = unsafe {
            ckb_contract_std::buddy_alloc::NonThreadsafeAlloc::new(_HEAP.as_ptr(), _HEAP_SIZE)
        };

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
