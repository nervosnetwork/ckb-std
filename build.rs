use std::env;

fn main() {
    println!("cargo:rerun-if-changed=c/dlopen.c");
    println!("cargo:rerun-if-changed=c/libc.c");

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    // ckb-std only supports riscv64 target arch
    // but we can still use cargo check under other archs
    if target_arch == "riscv64" && cfg!(feature = "dlopen-c") {
        let mut build = cc::Build::new();
        build
            .file("c/dlopen.c")
            .define("CKB_DECLARATION_ONLY", None);
        setup_compiler_flags(&mut build);
        build.include("c/ckb-c-stdlib");
        build.compile("dl-c-impl");
    }

    if target_arch == "riscv64" && cfg!(feature = "libc") {
        let mut build = cc::Build::new();
        build.file("c/libc.c").define("__SHARED_LIBRARY__", None);
        setup_compiler_flags(&mut build);
        build.compile("libc");
    }
}

fn setup_compiler_flags(build: &mut cc::Build) {
    build
        .static_flag(true)
        .flag("-fno-builtin-printf")
        .flag("-fno-builtin-memcmp")
        .flag("-nostdinc")
        .flag("-nostdlib")
        .flag("-fvisibility=hidden")
        .flag("-fdata-sections")
        .flag("-ffunction-sections")
        .flag("-Wall")
        .flag("-Werror")
        .flag("-Wno-unused-parameter")
        .flag("-Wno-nonnull")
        .include("c/ckb-c-stdlib/libc");

    let clang = match std::env::var_os("CLANG") {
        Some(val) => val,
        None => "clang-18".into(),
    };

    if cfg!(feature = "build-with-clang") {
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
}
