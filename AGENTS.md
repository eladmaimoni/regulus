
# AGENTS.md

# Languages and Frameworks 
This project contains applications and tools in the GNSS and computer vision domains. It utilizes various languages and frameworks:

### C++
used primarily to implement algorithmic layers. Utilize various libraries that are only available in C++ as first class libraries (such as opencv and others).
C++ may utilize rust code for things such as logging, profiling and other core infrastrucures. 
Generally, we will try to avoid using multithreading and perform IO in directly in C++.
The C++ codebase will consist of many static libraries (both inhouse and 3rd party) and will either be compiled to a single dynamic library or multiple static library to be consumed by other system components.

### Rust
used for system programming, networking and all things that can be done more easily without using native C++ libraries.
The codebase will contain:
- a core rust crate whose purpose is to expose rust utilities to C++ (so that for instance, code in both languages may use the same logging infrastruture)
- other rust crates that consume both the C++ library and rust crates. These can be executables or libraries.

# Cuda
Cuda is used for acceleration. primarily on Jetson devices.

### Flutter
Flutter will be used for UI and will communicate with the rust ecosystem via flutter_rust_bridge.

