#include "fc_internal.h"

fc_result fc_encoder_set_bitrate(fc_encoder* encoder, uint32_t bitrate_kbps)
{
    fc_settings settings;
    fc_result result;

    if (encoder == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }

    settings = encoder->settings;
    settings.bitrate_kbps = bitrate_kbps;
    result = fc_validate_settings(encoder, &settings);
    if (result.status != 0) {
        return result;
    }

    result = fc_reconfigure_bitrate(encoder, bitrate_kbps);
    if (result.status == 0) {
        encoder->settings = settings;
    }

    return result;
}
