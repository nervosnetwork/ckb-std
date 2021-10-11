extern crate alloc;


#[path = "../../../contracts/exec_caller_by_code_hash/src/entry.rs"]
mod entry;

fn main() {
    let code = entry::main() as i32;
    if code != 0 {
        println!("exit with {}", code);
    }
    std::process::exit(code);
}
