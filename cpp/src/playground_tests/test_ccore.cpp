#include <catch2/catch_test_macros.hpp>
#include <catch2/benchmark/catch_benchmark.hpp>

TEST_CASE("smoke test", "[ccore][smoke]") {
    REQUIRE(1 + 1 == 2);
}
