#include "fc_internal.h"

fc_result fc_open_session(fc_encoder* encoder)
{
    NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS params;
    NVENCSTATUS status;

    memset(&params, 0, sizeof(params));
    params.version = NV_ENC_OPEN_ENCODE_SESSION_EX_PARAMS_VER;
    params.deviceType = NV_ENC_DEVICE_TYPE_DIRECTX;
    params.device = encoder->device;
    params.apiVersion = NVENCAPI_VERSION;

    status = encoder->api.nvEncOpenEncodeSessionEx(&params, &encoder->session);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_OPEN_SESSION);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
