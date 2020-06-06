//! Mixed strategy allocator

use super::block_list_alloc::{BlockListAlloc, BLOCK_SIZE};
use buddy_alloc::NonThreadsafeAlloc as BuddyAlloc;
use core::alloc::{GlobalAlloc, Layout};

/// Use buddy allocator if request bytes is large than this,
/// otherwise use block list allocator
const BLOCK_ALLOC_LIMIT: usize = BLOCK_SIZE;

pub struct MixedAlloc {
    block_list_alloc: BlockListAlloc,
    buddy_alloc: BuddyAlloc,
}

impl MixedAlloc {
    pub const fn new(block_list_alloc: BlockListAlloc, buddy_alloc: BuddyAlloc) -> Self {
        MixedAlloc {
            block_list_alloc,
            buddy_alloc,
        }
    }
}

unsafe impl GlobalAlloc for MixedAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let bytes = layout.size();
        if bytes > BLOCK_ALLOC_LIMIT {
            self.buddy_alloc.alloc(layout)
        } else {
            let mut p = self.block_list_alloc.alloc(layout);
            if p.is_null() {
                p = self.buddy_alloc.alloc(layout);
            }
            p
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if self.block_list_alloc.contains_ptr(ptr) {
            self.block_list_alloc.dealloc(ptr, layout);
        } else {
            self.buddy_alloc.dealloc(ptr, layout);
        }
    }
}

unsafe impl Sync for MixedAlloc {}
