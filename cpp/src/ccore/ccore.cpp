#include "ccore/ccore.hpp"
#include "rcore_cxx/lib.h"

namespace rg
{
    int add_cpp(int a, int b)
    {
        initialize_tracing();
        trace_info("hi from cpp");
        return a + b;
    }
}