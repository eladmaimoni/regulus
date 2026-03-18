// disable unused warnings
#![allow(unused)]

#[cxx::bridge(namespace = "rg")]
mod ffi {
    unsafe extern "C++" {
        // include!("../../../cpp/src/ccore/ccore.hpp");
        include!("ccore/ccore.hpp");
        fn add_cpp(a: i32, b: i32) -> i32;
    }
}