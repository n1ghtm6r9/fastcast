#include "fc_internal.h"

void fc_configure_h264(NV_ENC_CONFIG_H264* h264, uint32_t bit_depth)
{
    NV_ENC_BIT_DEPTH depth = bit_depth == 10u ? NV_ENC_BIT_DEPTH_10 : NV_ENC_BIT_DEPTH_8;

    h264->idrPeriod = NVENC_INFINITE_GOPLENGTH;
    h264->repeatSPSPPS = 1;
    h264->inputBitDepth = depth;
    h264->outputBitDepth = depth;
    fc_set_vui(&h264->h264VUIParameters);
}
