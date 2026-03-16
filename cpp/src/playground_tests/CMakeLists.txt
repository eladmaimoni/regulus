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
    regulus_default_compiler_flags
)

target_compile_definitions(
   playground_tests
   PRIVATE
   CATCH_CONFIG_ENABLE_BENCHMARKING
)

