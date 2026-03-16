
fn main(){

    // This generates the C++ header and source files and places them in the target/cxxbridge
    // note that we don not call ::compile on the returned build object.
    // this is because we want to compile the bridge files from a cmake project.
    // reference https://github.com/XiangpengHao/cxx-cmake-example/blob/master/rust_part/build.rs
    let _bridge = cxx_build::bridge("src/lib.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");
}