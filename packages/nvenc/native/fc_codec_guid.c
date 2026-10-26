#include "fc_internal.h"

GUID fc_codec_guid(uint32_t codec)
{
    if (codec == FC_CODEC_AV1) {
        return NV_ENC_CODEC_AV1_GUID;
    }
    if (codec == FC_CODEC_HEVC) {
        return NV_ENC_CODEC_HEVC_GUID;
    }
    return NV_ENC_CODEC_H264_GUID;
}
