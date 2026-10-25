#include "fc_internal.h"

NV_ENC_BUFFER_FORMAT fc_buffer_format(uint32_t bit_depth)
{
    if (bit_depth == 10u) {
        return NV_ENC_BUFFER_FORMAT_YUV420_10BIT;
    }

    return NV_ENC_BUFFER_FORMAT_NV12;
}
