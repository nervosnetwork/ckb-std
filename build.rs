use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // ckb-std only supports riscv64 target arch
    // but we can still use cargo check under other archs
    if target_arch == "riscv64" && cfg!(feature = "dlopen-c") {
        cc::Build::new()
            .file("dl-c-impl/lib.c")
            .static_flag(true)
            .flag("-O3")
            .flag("-fno-builtin-printf")
            .flag("-fno-builtin-memcmp")
            .flag("-nostdinc")
            .flag("-nostdlib")
            .flag("-fvisibility=hidden")
            .flag("-fdata-sections")
            .flag("-ffunction-sections")
            .include("dl-c-impl/ckb-c-stdlib")
            .include("dl-c-impl/ckb-c-stdlib/libc")
            .flag("-Wall")
            .flag("-Werror")
            .flag("-Wno-unused-parameter")
            .flag("-Wno-nonnull")
            .define("__SHARED_LIBRARY__", None)
            .flag("-Wno-nonnull-compare")
            .flag("-nostartfiles")
            .flag("-Wno-dangling-pointer")
            .compile("dl-c-impl");
    }

    if target_arch == "riscv64" && target_os == "ckb" && (!cfg!(feature = "no-linking-dummy-libc"))
    {
        println!("cargo:rustc-link-lib=dummylibc");
    }
}
