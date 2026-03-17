
cmake_minimum_required(VERSION 4.3.0)

set(LOG_SEPARATOR "===============================================================")
project(regulus)
message(STATUS "${LOG_SEPARATOR}")
message(STATUS "Project: ${PROJECT_NAME}")
message(STATUS "${LOG_SEPARATOR}")

option(REGULUS_SHARED "Build one shared library instead of per-lib static libraries" OFF)

if(REGULUS_SHARED)
  message(STATUS "Building shared library")

  # Use position-independent code when building shared (dynamic) libraries,
  # or when building static libraries intended to be linked into shared libraries.
  set(CMAKE_POSITION_INDEPENDENT_CODE ON)

  # set_target_properties(alpha PROPERTIES POSITION_INDEPENDENT_CODE ON)
endif()

include(FetchContent)

include(cmake/version_details.cmake)
include(cmake/compiler_flags.cmake)
include(cmake/global_setup.cmake)
include(cmake/debugging_utils.cmake)
include(cmake/dynamic_library_link.cmake)
include(cmake/code_analysis.cmake)
include(cmake/target_helpers.cmake)
include(cmake/installation.cmake)

include(GNUInstallDirs)

# the export set that collects all targets to be exported
# using the install(TARGETS ... EXPORT ...) command
set(export_set "${PROJECT_NAME}_exported_targets")
add_target_to_global_export_set(regulus_default_compiler_flags)

# Use GNU install dirs for sensible defaults (CMAKE_INSTALL_LIBDIR / INCLUDEDIR)
find_package(Catch2 CONFIG REQUIRED)
find_package(Eigen3 CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)

FetchContent_Declare(
    Corrosion
    GIT_REPOSITORY https://github.com/corrosion-rs/corrosion.git
    GIT_TAG master
)
FetchContent_MakeAvailable(Corrosion)

corrosion_import_crate(MANIFEST_PATH ../rust/src/rcore/Cargo.toml)
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


corrosion_add_cxxbridge(rcore_cxx
    CRATE rcore
    FILES lib.rs
)
add_library(rcore::rcore ALIAS rcore)

include(CTest)
include(Catch)

add_subdirectory(src)

# Project-level installs (once per project).
install(EXPORT "${export_set}"
  NAMESPACE "${PROJECT_NAME}::"
  FILE "${PROJECT_NAME}Targets.cmake"
  DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}"
)