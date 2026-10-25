#include "fc_internal.h"

fc_result fc_check_bit_depth(fc_encoder* encoder)
{
    NV_ENC_CAPS_PARAM caps;
    NVENCSTATUS status;
    int supported = 0;

    if (encoder->settings.bit_depth != 10u) {
        return fc_result_of(0, FC_CALL_NONE);
    }

    memset(&caps, 0, sizeof(caps));
    caps.version = NV_ENC_CAPS_PARAM_VER;
    caps.capsToQuery = NV_ENC_CAPS_SUPPORT_10BIT_ENCODE;
    status = encoder->api.nvEncGetEncodeCaps(encoder->session, fc_codec_guid(encoder->settings.codec), &caps, &supported);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_GET_CAPS);
    }
    if (supported == 0) {
        return fc_result_of(FC_CODEC_UNSUPPORTED, FC_CALL_GET_CAPS);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
