pub mod tracing_facade;


pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


// CXX needs those functions to be visible at the crate root scope (not behind tracing_facade::). 
// We need to re-export them
pub use tracing_facade::{initialize_tracing, trace_info};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn initialize_tracing();
        fn trace_info(event: &str);
    }
}






#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
