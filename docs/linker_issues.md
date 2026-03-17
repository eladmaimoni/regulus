# Known Linker Issues

## Windows

First let's understand the flags responsible for linking the crt on msvc:

### /MD /MDd /MT /MTd
these are compiler flags (not linker flags!) that affect a few other things. for example /MDd does 2 things
- Defines _DEBUG and _DLL preprocessor macros (affecting _ITERATOR_DEBUG_LEVEL etc.)
- Embeds a /DEFAULTLIB:msvcrtd directive into the resulting .obj file, telling the linker to link this obj using a a debug version of the dynamic crt (dll).

### /DEFAULTLIB
this is a linker flag that might be specified by various targets (static libraries or binaries).
each target may specify a different thing. so for instance rust static libraries specify /DEFAUTLIB:msvcrt which means the library wishes to link the release version of the dynamic crt (MultithreadedDLL), equivalent to /MD.
when various targets specify different /DEFAULTLIB:<variant>, the linker may resolve those conflicts and decide on a different variant.
for example, if we have a C++ exe that links at debug version of the crt (/MDd that implies /DEFAUTLIB:msvcrtd) and rust library specifying a release version (/DEFAUTLIB:msvcrt) the linker might decide to simply link the debug version.
However, If we have a C++ exe that links the crt statically (/MT or /MTd), the linker won't be able to resolve those conflicts:

So in fact, on windows, rust (from version 1.79 according to [this PR](https://github.com/rust-lang/rust/pull/122268)) does not force the user to use a specific crt.

## CXX and CC crates
so there is not real issue with rust static library specifying different linker flags from a C++ dll / exe that links it.
however, if rust has a build.rs file that compiles some C++ code and specifes /MD, this will result in conflicts since /MD also affects some other compilers options such as preprocessor variables:
```
rcore.lib(15a3702a9d40a852-cxx.o) : error LNK2038: mismatch detected for '_ITERATOR_DEBUG_LEVEL': value '0' doesn't match value '2'
rcore.lib(15a3702a9d40a852-cxx.o) : error LNK2038: mismatch detected for 'RuntimeLibrary': value 'MD_DynamicRelease' doesn't match value 'MDd_DynamicDebug'
```
this error might be misleading at first since it may lead you to believe this is a linker flag issue.
however, this problem is that cxx crate (which uses cc crate) compiles some C++ code in its internal build.rs and embeds that into the rust artifact.
this C++ code (mostly things that interop with standard C++ types) are compiled with /MD by default.
While we can mix /DEFAULTLIB flags, we can't mix /MD with contradiciting compiler flags.
So here, the workaround is to tell cxx crate or cc crate to explicitly use the right flag using environment variables:
```
if(MSVC)
    string(FIND "${CMAKE_MSVC_RUNTIME_LIBRARY}" "DLL" _is_dynamic_crt)
    if(NOT _is_dynamic_crt EQUAL -1)
        message(STATUS "we are using a dynamic CRT, making sure cxx internally (in its build.rs) links the debug version of the CRT")
        corrosion_set_env_vars(rcore
            "CFLAGS=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
            "CXXFLAGS=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
        )
    else()
        message(STATUS "we are using a static CRT, making sure cxx internally (in its build.rs) links the release version of the CRT")
        corrosion_set_env_vars(rcore
            "CFLAGS=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
            "CXXFLAGS=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
        )
    endif()
endif()
```

see [here](https://github.com/dtolnay/cxx/issues/880) for reference. 

