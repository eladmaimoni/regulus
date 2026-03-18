

// #[derive(Debug)]
// pub struct PkgConfigLinkData {
//     pub link_search_paths: Vec<String>,
//     pub static_libs: Vec<String>,
//     pub dynamic_libs: Vec<String>,
// }

// fn get_link_search_paths(library_data: &pkg_config::LibraryData) -> Vec<String> {
//     library_data.link_dirs.iter().map(|path| path.display().to_string()).collect()
// }

// pub fn parse_pkg_config_file(pkg_config_file: &std::path::Path) -> PkgConfigLinkData {

//     unsafe {
//         std::env::set_var("PKG_CONFIG_PATH", pkg_config_file.display().to_string());
//     }

//     let library_data = pkg_config::Config::new().statik(true).probe("ccore").expect("failed to probe ccore pkg-config file");

// }

pub fn emit_pkg_config_link_data(pkg_config_file: &std::path::Path) {

    unsafe {
        std::env::set_var("PKG_CONFIG_PATH", pkg_config_file.display().to_string());
    }

    pkg_config::Config::new().statik(true).probe("ccore").expect("failed to probe ccore pkg-config file");

    
}