
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TargetArch {
    X86_64,
    Aarch64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TargetBuildProfile {
    Debug,
    Release,
}

#[derive(Debug)]
pub struct CMakePresets {
    pub(crate) configure: &'static str,
    pub(crate) build: &'static str,
}

pub const CMAKE_INSTALLED_DIR: &str = "installed";
pub const VCPKG_INSTALLED_DIR: &str = "vcpkg_installed";

pub fn get_target_os(target_os: &str) -> TargetOS {
    match target_os {
        "windows" => TargetOS::Windows,
        "linux" => TargetOS::Linux,
        "macos" => TargetOS::MacOS,
        _ => panic!("Unsupported target OS: {}", target_os),
    }
}

/// Returns the repository root (directory containing `Cargo.toml` and `CMakePresets.json`).
pub fn get_repository_root() -> std::path::PathBuf {
    let manifest_dir = std::path::PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    manifest_dir
        .ancestors()
        .nth(3)
        .expect("ccore_bridge must live at rust/src/ccore_bridge under the workspace root")
        .to_path_buf()
}

pub fn get_cpp_project_root_directory() -> std::path::PathBuf {
    get_repository_root().join("cpp")
}

pub fn get_cargo_out_dir() -> std::path::PathBuf {
    // this is set by cargo to the folder where the build script should place its output
    let out_dir = std::env::var("OUT_DIR").unwrap().replace('\\', "/");
    std::path::PathBuf::from(out_dir)
}

pub fn get_target_arch(target_arch: &str) -> TargetArch {
    match target_arch {
        "x86_64" => TargetArch::X86_64,
        "aarch64" => TargetArch::Aarch64,
        _ => panic!("Unsupported target architecture: {}", target_arch),
    }
}

pub fn get_cargo_target_build_profile() -> TargetBuildProfile {
    let profile = std::env::var("PROFILE").unwrap();
    match profile.as_str() {
        "debug" => TargetBuildProfile::Debug,
        "release" => TargetBuildProfile::Release,
        _ => panic!("Unsupported profile: {}", profile),
    }
}

const WINDOWS_DEFAULT_DEBUG_PRESET: CMakePresets = CMakePresets {
    configure: "msvc-mt",
    build: "msvc-mt-debug",
};

const WINDOWS_DEFAULT_RELEASE_PRESET: CMakePresets = CMakePresets {
    configure: "msvc-mt",
    build: "msvc-mt-release",
};

const LINUX_DEFAULT_DEBUG_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-debug",
    build: "clang-20-debug",
};

const LINUX_DEFAULT_RELEASE_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-release",
    build: "clang-20-release",
};

const LINUX_AARCH64_DEBUG_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-aarch64-debug",
    build: "clang-20-aarch64-debug",
};

const LINUX_AARCH64_RELEASE_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-aarch64-release",
    build: "clang-20-aarch64-release",
};

pub fn get_cmake_presets(
    target_os: TargetOS,
    target_arch: TargetArch,
    target_build_profile: TargetBuildProfile,
) -> CMakePresets {
    match (target_os, target_arch) {
        (TargetOS::Windows, TargetArch::X86_64) => match target_build_profile {
            TargetBuildProfile::Debug => WINDOWS_DEFAULT_DEBUG_PRESET,
            TargetBuildProfile::Release => WINDOWS_DEFAULT_RELEASE_PRESET,
        },
        (TargetOS::Linux, TargetArch::X86_64) => match target_build_profile {
            TargetBuildProfile::Debug => LINUX_DEFAULT_DEBUG_PRESET,
            TargetBuildProfile::Release => LINUX_DEFAULT_RELEASE_PRESET,
        },
        (TargetOS::Linux, TargetArch::Aarch64) => match target_build_profile {
            TargetBuildProfile::Debug => LINUX_AARCH64_DEBUG_PRESET,
            TargetBuildProfile::Release => LINUX_AARCH64_RELEASE_PRESET,
        },
        (TargetOS::MacOS, _) => {
            panic!("MacOS is not supported yet");
        }
        (os, arch) => {
            panic!("Unsupported OS/architecture combination: {:?}/{:?}", os, arch);
        }
    }
}

pub fn get_pkg_config_dir(cmake_install_path: &std::path::Path, build_profile: TargetBuildProfile) -> std::path::PathBuf {
    match build_profile {
        TargetBuildProfile::Debug => cmake_install_path.join("debug/lib/pkgconfig"),
        TargetBuildProfile::Release => cmake_install_path.join("lib/pkgconfig"),
    }
}