type CkbMainFunc<'a> = libloading::Symbol<
    'a,
    unsafe extern "C" fn(
        argc: core::ffi::c_int,
        // Arg is the same as *const c_char ABI wise.
        argv: *const ckb_std::env::Arg,
        // tx: *const core::ffi::c_char,
        // tx_len: core::ffi::c_int,
    ) -> i8,
>;

fn get_dylib_path(name: &str) -> String {
    let lib_dir = name.replace("_", "-");

    #[cfg(target_os = "macos")]
    let lib_name = format!("lib{}_dbg.dylib", name);

    #[cfg(target_os = "linux")]
    let lib_name =  format!("lib{}_dbg.so", name);

    format!(
        "../../contracts/{0}/{0}-dbg/target/debug/{1}",
        lib_dir, lib_name
    )
}

fn run_simulator<'a>(name: &str, args: &[&str]) -> Result<i8, ()> {
    let lib_path = get_dylib_path(name);
    unsafe {
        if let Ok(lib) = libloading::Library::new(lib_path) {
            if let Ok(func) = lib.get(b"__ckb_std_main") {
                let func: CkbMainFunc = func;
                let args: Vec<ckb_std::env::Arg> =
                    args.iter().map(|f| ckb_std::env::Arg::new(f)).collect();
                let rc = { func(args.len() as core::ffi::c_int, args.as_ptr()) };
                return Ok(rc);
            }
        }
        Err(())
    }
}

#[test]
fn test_exec_callee() {
    let r = run_simulator("exec_callee", &["Hello World\0", "你好\0"]);
    assert_eq!(r.unwrap(), 0);
}
