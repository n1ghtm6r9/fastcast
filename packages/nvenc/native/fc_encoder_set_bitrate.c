#include "fc_internal.h"

fc_result fc_encoder_set_bitrate(fc_encoder* encoder, uint32_t bitrate_kbps)
{
    fc_settings settings;
    fc_result validation;

    if (encoder == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }

    settings = encoder->settings;
    settings.bitrate_kbps = bitrate_kbps;
    validation = fc_validate_settings(encoder, &settings);
    if (validation.status != 0) {
        return validation;
    }

    return fc_result_of(FC_LIBRARY_MISSING, FC_CALL_LOAD);
}
