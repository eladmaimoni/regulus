// disable unused warnings
#![allow(unused)]

// Force-link rcore so its CXX bridge C++ glue (rcore_cxx.lib) is available.
// ccore.lib calls rcore's CXX bridge functions (initialize_tracing, trace_info)
// which are C++ wrappers compiled by rcore's build.rs. Without this, Cargo
// won't link librcore.rlib (and its native deps) since no Rust code uses it.
extern crate rcore;

#[cxx::bridge(namespace = "rg")]
mod ffi {
    unsafe extern "C++" {
        include!("ccore/ccore.hpp");
        fn add_cpp(a: i32, b: i32) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
    #[test]
    fn test_add_cpp() {
        let result = ffi::add_cpp(2, 3);
        assert_eq!(result, 5);
    }
}