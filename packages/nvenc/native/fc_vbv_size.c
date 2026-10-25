#include "fc_internal.h"

uint32_t fc_vbv_size(uint32_t bitrate_kbps, uint32_t fps)
{
    return (uint32_t)(((uint64_t)bitrate_kbps * 1000u) / fps);
}
