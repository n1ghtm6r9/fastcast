#include "fc_internal.h"

fc_result fc_initialize(fc_encoder* encoder)
{
    const fc_settings* settings = &encoder->settings;
    NV_ENC_INITIALIZE_PARAMS* init = &encoder->init;
    NVENCSTATUS status;

    memset(init, 0, sizeof(*init));
    init->version = NV_ENC_INITIALIZE_PARAMS_VER;
    init->encodeGUID = fc_codec_guid(settings->codec);
    init->presetGUID = fc_preset_guid(settings->preset);
    init->encodeWidth = settings->width;
    init->encodeHeight = settings->height;
    init->darWidth = settings->width;
    init->darHeight = settings->height;
    init->maxEncodeWidth = settings->width;
    init->maxEncodeHeight = settings->height;
    init->frameRateNum = settings->fps;
    init->frameRateDen = 1;
    init->enablePTD = 1;
    init->enableEncodeAsync = 0;
    init->tuningInfo = NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY;
    init->encodeConfig = &encoder->config;

    status = encoder->api.nvEncInitializeEncoder(encoder->session, init);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_INITIALIZE);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
