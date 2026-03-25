// disable unused warnings
#![allow(unused)]

#[path = "build/ccore_build_helper.rs"]
mod ccore_build_helper;
#[path = "build/pkg_config_parsing.rs"]
mod pkg_config_parsing;

use ccore_build_helper::*;
use pkg_config_parsing::*;

fn log_debug(
    cmake_presets: &CMakePresets,
    cpp_root: &std::path::Path,
    cmake_install_dir: &std::path::Path,
    vcpkg_install_dir: &std::path::Path,
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



fn build_ccore() {
    let target_os = get_target_os(std::env::var("CARGO_CFG_TARGET_OS").unwrap().as_str());
    let target_arch = get_target_arch(std::env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str());
    let target_build_profile = get_cargo_target_build_profile();
    let cmake_presets = get_cmake_presets(target_os, target_arch, target_build_profile);
    let cpp_root = get_cpp_project_root_directory();
    let project_root = get_repository_root();

    let cargo_out_dir = get_cargo_out_dir();
    let cmake_install_dir = cargo_out_dir.join(CMAKE_INSTALLED_DIR);
    let vcpkg_install_dir = cargo_out_dir.join(VCPKG_INSTALLED_DIR);

    log_debug(
        &cmake_presets,
        &cpp_root,
        &cmake_install_dir,
        &vcpkg_install_dir,
    );

    let status = std::process::Command::new("cmake")
        .current_dir(&project_root)
        .arg(format!("--preset={}", cmake_presets.configure))
        .arg(format!(
            "-DCMAKE_INSTALL_PREFIX={}",
            cmake_install_dir.display().to_string()
        ))
        .arg(format!(
            "-DVCPKG_INSTALLED_DIR={}",
            vcpkg_install_dir.display().to_string()
        ))
        .status()
        .expect("failed to run cmake configure");

    if !status.success() {
        panic!("failed to run cmake configure");
    }

    let status = std::process::Command::new("cmake")
        .current_dir(&project_root)
        .arg("--build")
        .arg(format!("--preset={}", cmake_presets.build))
        .arg("--target=ccore")
        .status()
        .expect("failed to run cmake build");

    if !status.success() {
        panic!("failed to run cmake build");
    }

    let status = std::process::Command::new("cmake")
        .current_dir(&project_root)
        .arg("--build")
        .arg(format!("--preset={}", cmake_presets.build))
        .arg("--target=install")
        .status()
        .expect("failed to run cmake install");

    if !status.success() {
        panic!("failed to run cmake install");
    }
}

fn build_ccore_cxxbridge() {
    let cargo_out_dir = get_cargo_out_dir();
    let cmake_install_dir = cargo_out_dir.join(CMAKE_INSTALLED_DIR);
    let mut build = cxx_build::bridge("src/lib.rs");
    build.include(cmake_install_dir.join("include"));
    build.compile("ccore_bridge_cxx");
}

fn emit_ccore_link_data() {
    let target_build_profile = get_cargo_target_build_profile();
    let cargo_out_dir = get_cargo_out_dir();
    let cmake_install_dir = cargo_out_dir.join(CMAKE_INSTALLED_DIR);
    let pkg_config_dir = get_pkg_config_dir(&cmake_install_dir, target_build_profile);
    emit_pkg_config_link_data(&pkg_config_dir);
}

fn main() {
    // Order matters for Linux static linking: the linker processes libraries
    // left-to-right and discards symbols not yet referenced. We must emit
    // link directives so that -lccore_bridge_cxx (which references rg::add_cpp)
    // appears before -lccore (which defines it). MSVC rescans all libraries
    // so order is irrelevant there, but this ordering is correct for both.
    build_ccore();
    build_ccore_cxxbridge();
    emit_ccore_link_data();
}