#include <catch2/catch_test_macros.hpp>
#include <catch2/benchmark/catch_benchmark.hpp>
#include <ccore/ccore.hpp>

TEST_CASE("smoke test", "[ccore][smoke]") {
    auto result = rg::add_cpp(1, 2);
    REQUIRE(result == 3);
}
