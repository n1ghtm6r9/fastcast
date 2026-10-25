#include "fc_internal.h"

void fc_set_rate_control(NV_ENC_RC_PARAMS* rate_control, uint32_t bitrate_kbps, uint32_t fps)
{
    uint32_t bits_per_second = bitrate_kbps * 1000u;
    uint32_t vbv_size = fc_vbv_size(bitrate_kbps, fps);

    rate_control->rateControlMode = NV_ENC_PARAMS_RC_CBR;
    rate_control->averageBitRate = bits_per_second;
    rate_control->maxBitRate = bits_per_second;
    rate_control->vbvBufferSize = vbv_size;
    rate_control->vbvInitialDelay = vbv_size;
}
