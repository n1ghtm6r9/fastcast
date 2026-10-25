#include "fc_internal.h"

GUID fc_profile_guid(uint32_t codec, uint32_t bit_depth)
{
    if (codec == FC_CODEC_HEVC) {
        return bit_depth == 10u ? NV_ENC_HEVC_PROFILE_MAIN10_GUID : NV_ENC_HEVC_PROFILE_MAIN_GUID;
    }
    return bit_depth == 10u ? NV_ENC_H264_PROFILE_HIGH_10_GUID : NV_ENC_H264_PROFILE_HIGH_GUID;
}
