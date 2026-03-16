

set(REGULUS_LIBRARY_TYPE STATIC)
set(REGULUS_LIBRARY_LINKAGE PUBLIC)

if(REGULUS_SHARED)
    set(REGULUS_LIBRARY_TYPE SHARED)
    set(REGULUS_LIBRARY_LINKAGE PRIVATE)
endif()

add_library(
    ccore
    ${BY2_LIBRARY_TYPE}
    ccore/ccore.hpp
    ccore.cpp
)