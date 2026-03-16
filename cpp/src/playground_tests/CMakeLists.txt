set_source_group(
    run 
    main.cpp
)
set_source_group(
    tests 
    test_ccore.cpp
)
add_executable(playground_tests ${run} ${tests})

target_link_libraries(
    playground_tests
    PRIVATE
    ccore
    Catch2::Catch2
    regulus_default_compiler_flags
)

catch_discover_tests(playground_tests)

