function(import_rcore_target)
    FetchContent_Declare(
        Corrosion
        GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
        GIT_TAG master
    )
    FetchContent_MakeAvailable(Corrosion)

    corrosion_import_crate(MANIFEST_PATH ${CMAKE_CURRENT_SOURCE_DIR}/../rust/src/rcore/Cargo.toml)
    if(NOT TARGET rcore)
        message(FATAL_ERROR "rcore target not found")
    endif()

    if(WIN32)
        # On Windows, we need to link to the bcrypt libraries
        # to get the necessary functionality for the rcore crate.
        target_link_libraries(rcore INTERFACE bcrypt)


        # get_target_property(_rcore_type rcore TYPE)
        # if(_rcore_type STREQUAL "STATIC_LIBRARY")
        #     message(STATUS "rcore target is a static library")
        # else()
        #     message(STATUS "rcore target is not a static library: ${_rcore_type}")
        # endif()
    endif()

    if(MSVC)
        string(FIND "${CMAKE_MSVC_RUNTIME_LIBRARY}" "DLL" _is_dynamic_crt)
        if(NOT _is_dynamic_crt EQUAL -1)
            message(STATUS "we are using a dynamic CRT, making sure cxx internally (in its build.rs) links the debug version of the CRT")
            corrosion_set_env_vars(rcore
                "CFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
                "CXXFLAGS_x86_64_pc_windows_msvc=$<IF:$<CONFIG:Debug>,/MDd,/MD>"
            )
        else()
            target_link_options(rcore INTERFACE
                $<$<AND:$<CXX_COMPILER_ID:MSVC>,$<CONFIG:Debug>>:/NODEFAULTLIB:libcmt>
            )
            message(STATUS "we are using a static CRT, making sure cxx internally (in its build.rs) links the release version of the CRT")
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