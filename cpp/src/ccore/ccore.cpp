#include "ccore/ccore.hpp"
#include "rcore_cxx/lib.h"
#include <spdlog/spdlog.h>

namespace rg
{
    int add_cpp(int a, int b)
    {
        initialize_tracing();
        trace_info("hi from cpp");
        spdlog::info("hi from cpp spdlog");
        return a + b;
    }
}