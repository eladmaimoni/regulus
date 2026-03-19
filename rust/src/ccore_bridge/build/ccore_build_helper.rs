
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TargetOS {
    Windows,
    Linux,
    MacOS,
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
    pub(crate) install: &'static str,
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

pub fn get_repository_root() -> std::path::PathBuf {
    let current_directory = std::env::current_dir().unwrap(); // the build script’s current directory is the source directory of the build script’s package

    // Determine workspace root (two levels up from this crate: .../src/app)
    // let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    current_directory
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}


pub fn get_cpp_project_root_directory() -> std::path::PathBuf {
    get_repository_root().parent().unwrap().join("cpp")
}

pub fn get_cargo_out_dir() -> std::path::PathBuf {
    // this is set by cargo to the folder where the build script should place its output
    let out_dir = std::env::var("OUT_DIR").unwrap().replace('\\', "/");
    std::path::PathBuf::from(out_dir)
}

pub fn get_cargo_target_build_architecture() -> std::string::String {
    std::env::var("CARGO_CFG_TARGET_ARCH").expect("CARGO_CFG_TARGET_ARCH is not set")
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
    install: "msvc-mt-debug-install",
};

const WINDOWS_DEFAULT_RELEASE_PRESET: CMakePresets = CMakePresets {
    configure: "msvc-mt",
    build: "msvc-mt-release",
    install: "msvc-mt-release-install",
};

const LINUX_DEFAULT_DEBUG_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-debug",
    build: "clang-20-debug",
    install: "clang-20-debug-install",
};

const LINUX_DEFAULT_RELEASE_PRESET: CMakePresets = CMakePresets {
    configure: "clang-20-release",
    build: "clang-20-release",
    install: "clang-20-release-install",
};

pub fn get_cmake_presets(target_os: TargetOS, target_build_profile: TargetBuildProfile) -> CMakePresets {
    match target_os {
        TargetOS::Windows => match target_build_profile {
            TargetBuildProfile::Debug => WINDOWS_DEFAULT_DEBUG_PRESET,
            TargetBuildProfile::Release => WINDOWS_DEFAULT_RELEASE_PRESET,
        },
        TargetOS::Linux => match target_build_profile {
            TargetBuildProfile::Debug => LINUX_DEFAULT_DEBUG_PRESET,
            TargetBuildProfile::Release => LINUX_DEFAULT_RELEASE_PRESET,
        },
        TargetOS::MacOS => {
            panic!("MacOS is not supported yet");
        }
    }
}

pub fn get_pkg_config_dir(cmake_install_path: &std::path::Path, build_profile: TargetBuildProfile) -> std::path::PathBuf {
    match build_profile {
        TargetBuildProfile::Debug => cmake_install_path.join("debug/lib/pkgconfig"),
        TargetBuildProfile::Release => cmake_install_path.join("lib/pkgconfig"),
    }
}