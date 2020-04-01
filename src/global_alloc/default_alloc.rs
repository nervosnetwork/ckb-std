/// Usage
///
/// ``` rust
/// // define global allocator with defaut pure rust allocator
/// default_alloc!()
/// // indicate the heap size(default heap size is 64KB, with 16Bytes min allocated memory)
/// default_alloc!(64 * 1024, 16)
/// ```
#[macro_export]
macro_rules! default_alloc {
    () => {
        default_alloc!(64 * 1024, 16);
    };
    ($heap_size:expr, $min_block_size:expr) => {
        const _HEAP_SIZE: usize = $heap_size;
        static mut _HEAP: [u8; _HEAP_SIZE] = [0u8; _HEAP_SIZE];

        #[global_allocator]
        static ALLOC: $crate::buddy_alloc::NonThreadsafeAlloc = unsafe {
            $crate::buddy_alloc::NonThreadsafeAlloc::new(
                _HEAP.as_ptr(),
                _HEAP_SIZE,
                $min_block_size,
            )
        };
    };
}
