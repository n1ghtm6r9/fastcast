#include "fc_internal.h"

fc_result fc_reconfigure_bitrate(fc_encoder* encoder, uint32_t bitrate_kbps)
{
    NV_ENC_CONFIG config;
    NV_ENC_RECONFIGURE_PARAMS reconfigure;
    NVENCSTATUS status;

    config = encoder->config;
    fc_set_rate_control(&config.rcParams, bitrate_kbps, encoder->settings.fps);

    memset(&reconfigure, 0, sizeof(reconfigure));
    reconfigure.version = NV_ENC_RECONFIGURE_PARAMS_VER;
    reconfigure.reInitEncodeParams = encoder->init;
    reconfigure.reInitEncodeParams.encodeConfig = &config;
    reconfigure.resetEncoder = 0;
    reconfigure.forceIDR = 0;

    status = encoder->api.nvEncReconfigureEncoder(encoder->session, &reconfigure);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_RECONFIGURE);
    }

    encoder->config = config;
    return fc_result_of(0, FC_CALL_NONE);
}
