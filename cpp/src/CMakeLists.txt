add_subdirectory(ccore)

if(REGULUS_WITH_TESTS)
    add_subdirectory(playground_tests)
endif()