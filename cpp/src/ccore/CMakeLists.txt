

set(REGULUS_LIBRARY_TYPE STATIC)
set(REGULUS_LIBRARY_LINKAGE PUBLIC)

# if(REGULUS_SHARED)
#     set(REGULUS_LIBRARY_TYPE SHARED)
#     set(REGULUS_LIBRARY_LINKAGE PRIVATE)
# endif()

set_source_group(ccore ccore/ccore.hpp ccore.cpp)

add_library(
    ccore
    ${REGULUS_LIBRARY_TYPE}
    ${ccore}
)

target_link_libraries(
    ccore
    PUBLIC
    $<BUILD_INTERFACE:rcore_cxx>
    $<BUILD_INTERFACE:rcore::rcore>
    spdlog::spdlog
    regulus_default_compiler_flags
)

setup_target_includes_for_install(ccore ccore)
install_target_artifacts(ccore)
generate_pkgconfig(ccore)