extern crate alloc;

#[path = "../../../contracts/ckb-std-tests/src/entry.rs"]
mod entry;

fn main() {
    let code = entry::main() as i32;
    if code != 0 {
        println!("exit with {}", code);
    }
    std::process::exit(code);
}
