use ckb_allocator::bitmap_alloc::GlobalBitmapAlloc;

// This allocator can't work in tests since it's non-threadsafe.
#[cfg_attr(not(test), global_allocator)]
static ALLOC: GlobalBitmapAlloc = GlobalBitmapAlloc::new();

fn main() {
    let v = vec![0u8; 32];
    drop(v);
    let p1 = vec![0u8; 4096];
    let p2 = vec![0u8; 138];
    drop(p1);
    let msg = "alloc success".to_string();
    println!("{} {:?}", msg, p2.len());
    println!("hello");
}
