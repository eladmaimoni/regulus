function(import_rcore_target)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG master
    )
    FetchContent_MakeAvailable(Corrosion)

    corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/../rust/rcore/Cargo.toml)
    if(NOT TARGET rcore)
        message(FATAL_ERROR "rcore target not found")
    endif()

    if(WIN32)
        target_link_libraries(rcore INTERFACE bcrypt)
    endif()

    if(MSVC)
        string(FIND "${CMAKE_MSVC_RUNTIME_LIBRARY}" "DLL" _is_dynamic_crt)
        if(NOT _is_dynamic_crt EQUAL -1)
            message(STATUS "Dynamic CRT: setting CFLAGS so cxx's cc-compiled C++ matches /MD or /MDd per config")
            corrosion_set_env_vars(rcore
                "CFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
                "CXXFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
            )
        else()
            target_link_options(rcore INTERFACE
                $<$<AND:$<CXX_COMPILER_ID:MSVC>,$<CONFIG:Debug>>:/NODEFAULTLIB:libcmt>
            )
            message(STATUS "Static CRT: setting CFLAGS so cxx's cc-compiled C++ matches /MT or /MTd per config")
            corrosion_set_env_vars(rcore
                "CFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
                "CXXFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MTd,/MT>"
            )
        endif()
    endif()


    corrosion_add_cxxbridge(rcore_cxx
        CRATE rcore
        FILES lib.rs
    )
    add_library(rcore::rcore ALIAS rcore)
endfunction()