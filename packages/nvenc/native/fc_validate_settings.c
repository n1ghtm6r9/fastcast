#include "fc_internal.h"

fc_result fc_validate_settings(const void* owner, const fc_settings* settings)
{
    if (owner == NULL || settings == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }

    if ((settings->codec == FC_CODEC_H264 || settings->codec == FC_CODEC_HEVC)
        && settings->width >= FC_MIN_DIMENSION && settings->width <= FC_MAX_DIMENSION
        && settings->width % 2u == 0u
        && settings->height >= FC_MIN_DIMENSION && settings->height <= FC_MAX_DIMENSION
        && settings->height % 2u == 0u
        && settings->fps >= 1u && settings->fps <= FC_MAX_FPS
        && settings->bitrate_kbps >= 1u && settings->bitrate_kbps <= FC_MAX_BITRATE_KBPS
        && (settings->bit_depth == 8u || settings->bit_depth == 10u)
        && settings->preset >= FC_MIN_PRESET && settings->preset <= FC_MAX_PRESET) {
        return fc_result_of(0, FC_CALL_NONE);
    }

    return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
}
