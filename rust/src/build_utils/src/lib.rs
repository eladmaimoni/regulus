/// On MSVC debug builds, the C++ objects (compiled with /MTd via CFLAGS)
/// embed /DEFAULTLIB:libcmtd (static debug CRT). Rust's +crt-static
/// target feature makes Rust embed /DEFAULTLIB:libcmt (static release CRT).
/// These conflict: the linker drops libcmtd and debug CRT symbols like
/// _CrtDbgReport become unresolved. Override to use the debug variant.
pub fn emit_msvc_debug_crt_overrides() {
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        let profile = std::env::var("PROFILE").unwrap_or_default();
        if profile == "debug" {
            println!("cargo:rustc-link-arg=/nodefaultlib:libcmt");
            println!("cargo:rustc-link-arg=/defaultlib:libcmtd");
        }
    }
}
