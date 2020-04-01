use alloc::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

pub struct LibCAlloc;

unsafe impl GlobalAlloc for LibCAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        malloc(layout.size())
    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        free(ptr)
    }
}
