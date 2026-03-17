# Known Linker Issues

## Windows MSVC: CRT Linking with Rust Static Libraries

### Background: /MD /MDd /MT /MTd

These are **compiler** flags (not linker flags). They affect two things at compile time:

1. Define preprocessor macros that control debug/release behavior:
   - `/MDd` defines `_DEBUG` and `_DLL`, setting `_ITERATOR_DEBUG_LEVEL=2`
   - `/MD` defines `_DLL` only, setting `_ITERATOR_DEBUG_LEVEL=0`
   - `/MTd` and `/MT` are similar but for static CRT linkage
2. Embed a `/DEFAULTLIB:<variant>` directive into each `.obj` file, telling the linker which CRT variant the object expects:

| Flag   | Embedded directive       | CRT variant             |
|--------|--------------------------|-------------------------|
| `/MD`  | `/DEFAULTLIB:msvcrt`     | Dynamic release         |
| `/MDd` | `/DEFAULTLIB:msvcrtd`    | Dynamic debug           |
| `/MT`  | `/DEFAULTLIB:libcmt`     | Static release          |
| `/MTd` | `/DEFAULTLIB:libcmtd`    | Static debug            |

### Background: /DEFAULTLIB

`/DEFAULTLIB` is a linker directive (often embedded in `.obj` and `.lib` files, not passed on the command line).
When multiple objects specify conflicting `/DEFAULTLIB` variants within the same CRT family (e.g., `msvcrt` vs `msvcrtd`),
the linker can usually resolve the conflict with a warning (`LNK4098`).
However, mixing static and dynamic CRT families (e.g., `msvcrt` with `libcmt`) is unsupported by Microsoft and will typically fail.

### How Rust links the CRT

Since **Rust 1.79** ([PR #122268](https://github.com/rust-lang/rust/pull/122268)), Rust embeds `/DEFAULTLIB:msvcrt`
(or `/DEFAULTLIB:libcmt` if `+crt-static` target feature is set) into its objects.
Because this is a `/DEFAULTLIB` directive (not an explicit lib), it can be overridden by the consuming linker.

This means Rust itself does **not** force a specific CRT variant — the final executable's CRT choice is determined
by the C++ compiler flags and the linker's conflict resolution.

### The actual problem: C++ objects compiled by the `cxx` crate

The `cxx` crate (dependency, not `cxx-build`) has an internal `build.rs` that uses the `cc` crate to compile
C++ runtime code (`cxx.cc` — implementations of `rust::String`, `rust::Vec`, etc.).
These compiled `.obj` files are embedded inside the Rust static library (`rcore.lib`).

By default, the `cc` crate compiles with `/MD` (dynamic release CRT). This embeds two things into those objects:
- `RuntimeLibrary = MD_DynamicRelease`
- `_ITERATOR_DEBUG_LEVEL = 0`

When linking a C++ Debug executable (compiled with `/MDd`), the linker detects a **metadata mismatch**:

```
rcore.lib(cxx.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2'
rcore.lib(cxx.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug'
```

This is **not** a linker flag conflict — it is a compile-time ABI mismatch baked into the `.obj` files.
The `/DEFAULTLIB` conflict can be resolved by the linker, but `_ITERATOR_DEBUG_LEVEL` and `RuntimeLibrary`
metadata mismatches cannot — the linker refuses to link objects compiled with incompatible settings.

### Workaround

The `cc` crate reads `CFLAGS` and `CXXFLAGS` from the environment. By setting these before cargo builds,
we can make the C++ objects inside the Rust library match the consuming project's CRT configuration.

Using [Corrosion](https://github.com/corrosion-rs/corrosion) in CMake:

```cmake
if(MSVC)
    string(FIND "${CMAKE_MSVC_RUNTIME_LIBRARY}" "DLL" _is_dynamic_crt)
    if(NOT _is_dynamic_crt EQUAL -1)
        corrosion_set_env_vars(rcore
            "CFLAGS=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
            "CXXFLAGS=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
        )
    else()
        corrosion_set_env_vars(rcore
            "CFLAGS=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
            "CXXFLAGS=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
        )
    endif()
endif()
```

This uses generator expressions so the correct flag is applied per build configuration
(Debug vs Release) in multi-config generators like Visual Studio.

### Windows system library dependencies

When linking a Rust `staticlib` into a C++ executable, Windows system libraries used by Rust dependencies
are not automatically linked. You must add them explicitly:

```cmake
if(WIN32)
    target_link_libraries(rcore INTERFACE bcrypt advapi32)
endif()
```

Common system libraries Rust crates may need: `bcrypt`, `advapi32`, `userenv`, `ws2_32`, `ntdll`.

### Requirements

- **Rust >= 1.79** for the `/DEFAULTLIB`-based CRT linking (otherwise CRT is hardcoded and cannot be overridden).
- **Rust >= 1.83** recommended (fixes edge cases with CRT override when cargo explicitly inserts `msvcrt.lib`).

### References

- [CXX issue #880: Document how to get a MSVCRTD-based debug-build LIB on Windows](https://github.com/dtolnay/cxx/issues/880)
- [Rust PR #122268: Link MSVC default lib in core](https://github.com/rust-lang/rust/pull/122268)
- [Rust issue #107570: The MSVC CRT library is not overrideable by other build systems](https://github.com/rust-lang/rust/issues/107570)
- [Rust libs-team issue #211: Windows MSVC CRT linking](https://github.com/rust-lang/libs-team/issues/211)
