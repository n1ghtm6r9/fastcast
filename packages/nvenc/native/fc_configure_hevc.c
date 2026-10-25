#include "fc_internal.h"

void fc_configure_hevc(NV_ENC_CONFIG_HEVC* hevc, uint32_t bit_depth)
{
    NV_ENC_BIT_DEPTH depth = bit_depth == 10u ? NV_ENC_BIT_DEPTH_10 : NV_ENC_BIT_DEPTH_8;

    hevc->idrPeriod = NVENC_INFINITE_GOPLENGTH;
    hevc->repeatSPSPPS = 1;
    hevc->inputBitDepth = depth;
    hevc->outputBitDepth = depth;
    fc_set_vui(&hevc->hevcVUIParameters);
}
