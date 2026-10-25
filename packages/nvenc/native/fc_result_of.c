#include "fc_internal.h"

fc_result fc_result_of(int32_t status, uint32_t call)
{
    fc_result result;

    result.status = status;
    result.call = call;
    return result;
}
