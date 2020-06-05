//! Mixed strategy allocator

use super::bitmap_alloc::{BitmapAlloc, MIN_MEMORY_BLOCK};
use buddy_alloc::NonThreadsafeAlloc as BuddyAlloc;
use core::alloc::{GlobalAlloc, Layout};

/// Use buddy allocator if request bytes is large than this,
/// otherwise use bitmap allocator
const BITMAP_ALLOC_LIMIT: usize = MIN_MEMORY_BLOCK;

/// To break through the const function limitation
/// we receive BitmapAlloc as a pointer.
pub struct MixedAlloc {
    bitmap_alloc_ptr: *mut BitmapAlloc,
    buddy_alloc: BuddyAlloc,
}

impl MixedAlloc {
    pub const fn new(bitmap_alloc_ptr: *const BitmapAlloc, buddy_alloc: BuddyAlloc) -> Self {
        MixedAlloc {
            bitmap_alloc_ptr: bitmap_alloc_ptr as *mut BitmapAlloc,
            buddy_alloc,
        }
    }

    unsafe fn bitmap_alloc(&self) -> &mut BitmapAlloc {
        self.bitmap_alloc_ptr.as_mut().unwrap()
    }
}

unsafe impl GlobalAlloc for MixedAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let bytes = layout.size();
        if bytes > BITMAP_ALLOC_LIMIT {
            self.buddy_alloc.alloc(layout)
        } else {
            let mut p = self.bitmap_alloc().malloc(bytes);
            if p.is_null() {
                p = self.buddy_alloc.alloc(layout);
            }
            p
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.bitmap_alloc().contains_ptr(ptr) {
            self.bitmap_alloc().free(ptr, layout.size());
        } else {
            self.buddy_alloc.dealloc(ptr, layout);
        }
    }
}

unsafe impl Sync for MixedAlloc {}
