#include "fc_internal.h"

fc_result fc_configure(fc_encoder* encoder)
{
    const fc_settings* settings = &encoder->settings;
    NV_ENC_CONFIG* config = &encoder->config;
    uint32_t bits_per_second = settings->bitrate_kbps * 1000u;
    uint32_t vbv_size = fc_vbv_size(settings->bitrate_kbps, settings->fps);
    NV_ENC_PRESET_CONFIG preset;
    NVENCSTATUS status;

    memset(&preset, 0, sizeof(preset));
    preset.version = NV_ENC_PRESET_CONFIG_VER;
    preset.presetCfg.version = NV_ENC_CONFIG_VER;
    status = encoder->api.nvEncGetEncodePresetConfigEx(
        encoder->session,
        fc_codec_guid(settings->codec),
        fc_preset_guid(settings->preset),
        NV_ENC_TUNING_INFO_ULTRA_LOW_LATENCY,
        &preset);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_PRESET_CONFIG);
    }

    *config = preset.presetCfg;
    config->version = NV_ENC_CONFIG_VER;
    config->profileGUID = fc_profile_guid(settings->codec, settings->bit_depth);
    config->gopLength = NVENC_INFINITE_GOPLENGTH;
    config->frameIntervalP = 1;
    config->rcParams.rateControlMode = NV_ENC_PARAMS_RC_CBR;
    config->rcParams.averageBitRate = bits_per_second;
    config->rcParams.maxBitRate = bits_per_second;
    config->rcParams.vbvBufferSize = vbv_size;
    config->rcParams.vbvInitialDelay = vbv_size;

    if (settings->codec == FC_CODEC_HEVC) {
        fc_configure_hevc(&config->encodeCodecConfig.hevcConfig, settings->bit_depth);
    } else {
        fc_configure_h264(&config->encodeCodecConfig.h264Config, settings->bit_depth);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
