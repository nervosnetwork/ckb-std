use std::env;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    // ckb-std only supports riscv64 target arch
    // but we can still use cargo check under other archs
    if target_arch == "riscv64" && cfg!(feature = "dlopen-c") {
        let mut build = cc::Build::new();
        build
            .file("dl-c-impl/lib.c")
            .static_flag(true)
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
            .define("__SHARED_LIBRARY__", None);

        if cfg!(feature = "build-with-clang") {
            let clang = match std::env::var_os("CLANG") {
                Some(val) => val,
                None => "clang-16".into(),
            };

            build.compiler(clang);
        }

        let compiler = build.get_compiler();
        if compiler.is_like_clang() {
            build
                .no_default_flags(true)
                .flag("--target=riscv64")
                .flag("-march=rv64imc_zba_zbb_zbc_zbs");

            if env::var("DEBUG").map(|v| v != "false").unwrap_or(false) {
                build.flag("-g").flag("-fno-omit-frame-pointer");
            }

            let opt_level = env::var("OPT_LEVEL").expect("fetching OPT_LEVEL");
            if opt_level == "z" {
                build.flag("-Os");
            } else {
                build.flag(&format!("-O{}", opt_level));
            }
        } else if compiler.is_like_gnu() {
            build
                .flag("-nostartfiles")
                .flag("-Wno-dangling-pointer")
                .flag("-Wno-nonnull-compare");
        }

        build.compile("dl-c-impl");
    }

    if target_arch == "riscv64" && target_os == "ckb" && cfg!(feature = "dummy-libc") {
        println!("cargo:rustc-link-lib=dummylibc");
    }
}
