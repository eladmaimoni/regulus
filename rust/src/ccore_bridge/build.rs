// disable unused warnings
#![allow(unused)]

#[path = "build/ccore_build_helper.rs"]
mod ccore_build_helper;

use ccore_build_helper::*;

fn log_debug(
    cmake_presets: &CMakePresets,
    cpp_root: &std::path::Path,
    cmake_install_dir: &str,
    vcpkg_install_dir: &str,
) {
    // note that cargo does not allow multiline strings in println!

    println!("cargo:warning=debug: ccore_bridge build script:");
    println!("cargo:warning=debug: cmake presets = {:?}", cmake_presets);
    println!("cargo:warning=debug: cpp_root = {:?}", cpp_root);
    println!(
        "cargo:warning=debug: cmake_install_dir = {:?}",
        cmake_install_dir
    );
    println!(
        "cargo:warning=debug: vcpkg_install_dir = {:?}",
        vcpkg_install_dir
    );
}

fn main() {
    let target_os = get_target_os(std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str());
    let target_build_profile = get_cargo_target_build_profile();
    let cmake_presets = get_cmake_presets(target_os, target_build_profile);
    let cpp_root = get_cpp_project_root_directory();

    // determine where to place cmake installed and vcpkg installed files
    let cargo_out_dir = get_cargo_out_dir();
    let cmake_install_dir = cargo_out_dir
        .join(CMAKE_INSTALLED_DIR)
        .display()
        .to_string();
    let vcpkg_install_dir = cargo_out_dir
        .join(VCPKG_INSTALLED_DIR)
        .display()
        .to_string();

    log_debug(
        &cmake_presets,
        &cpp_root,
        &cmake_install_dir,
        &vcpkg_install_dir,
    );

    // run cmake configure
    let status = std::process::Command::new("cmake")
        .current_dir(&cpp_root)
        .arg(format!("--preset={}", cmake_presets.configure))
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", cmake_install_dir))
        .arg(format!("-DVCPKG_INSTALLED_DIR={}", vcpkg_install_dir))
        .status()
        .expect("failed to run cmake configure");

    if !status.success() {
        panic!("failed to run cmake configure");
    }

    // run cmake build
    let status = std::process::Command::new("cmake")
        .current_dir(&cpp_root)
        .arg("--build")
        .arg(format!("--preset={}", cmake_presets.build))
        .status()
        .expect("failed to run cmake build");

    if !status.success() {
        panic!("failed to run cmake build");
    }
}
