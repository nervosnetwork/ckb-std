/// Defines global allocator
///
///
/// # Example
///
/// ```
/// // define global allocator
/// default_alloc!()
///
/// // Default allocator uses a mixed allocation strategy:
/// // Fixed memory block area(64K) + Dynamic memory block area(64K)
/// // To customize the dynamic memory block heap size(default heap size is 64KB, min allocated memory is 64B)
/// default_alloc!(64 * 1024, 64)
/// ```
#[macro_export]
macro_rules! default_alloc {
    () => {
        default_alloc!(64 * 1024, 64);
    };
    ($heap_size:expr, $min_block_size:expr) => {
        static mut _BUDDY_HEAP: [u8; $heap_size] = [0u8; $heap_size];
        static mut BITMAP_ALLOC: $crate::ckb_allocator::bitmap_alloc::BitmapAlloc =
            $crate::ckb_allocator::bitmap_alloc::BitmapAlloc::default();

        #[global_allocator]
        static ALLOC: $crate::ckb_allocator::mixed_alloc::MixedAlloc = unsafe {
            let buddy_alloc = $crate::ckb_allocator::buddy_alloc::NonThreadsafeAlloc::new(
                _BUDDY_HEAP.as_ptr(),
                $heap_size,
                $min_block_size,
            );
            let bitmap_alloc_ptr =
                &BITMAP_ALLOC as *const $crate::ckb_allocator::bitmap_alloc::BitmapAlloc;
            $crate::ckb_allocator::mixed_alloc::MixedAlloc::new(bitmap_alloc_ptr, buddy_alloc)
        };
    };
}
