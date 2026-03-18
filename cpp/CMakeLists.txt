
cmake_minimum_required(VERSION 4.3.0)

set(LOG_SEPARATOR "===============================================================")
project(regulus)
message(STATUS "${LOG_SEPARATOR}")
message(STATUS "Project: ${PROJECT_NAME}")
message(STATUS "${LOG_SEPARATOR}")

option(REGULUS_SHARED "Build one shared library instead of per-lib static libraries" OFF)
option(REGULUS_WITH_RCORE "Build with rcore" ON)
option(REGULUS_WITH_TESTS "Build with tests" ON)

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
include(cmake/import_rcore_target.cmake)

include(GNUInstallDirs)

# the export set that collects all targets to be exported
# using the install(TARGETS ... EXPORT ...) command
set(export_set "${PROJECT_NAME}_exported_targets")
install_target_artifacts(regulus_default_compiler_flags)

# Use GNU install dirs for sensible defaults (CMAKE_INSTALL_LIBDIR / INCLUDEDIR)
find_package(Catch2 CONFIG REQUIRED)
find_package(Eigen3 CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)

if (REGULUS_WITH_RCORE)
    import_rcore_target()
endif()

include(CTest)
include(Catch)

add_subdirectory(src)

# Project-level installs (once per project).
# install(EXPORT "${export_set}"
#   NAMESPACE "${PROJECT_NAME}::"
#   FILE "${PROJECT_NAME}Targets.cmake"
#   DESTINATION "${CMAKE_INSTALL_LIBDIR}/cmake/${PROJECT_NAME}"
# )