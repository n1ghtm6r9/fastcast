#include "fc_internal.h"

fc_result fc_encoder_open(void* d3d11_device, const fc_settings* settings, fc_encoder** encoder)
{
    fc_result validation;

    if (encoder == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }
    *encoder = NULL;

    validation = fc_validate_settings(d3d11_device, settings);
    if (validation.status != 0) {
        return validation;
    }

    return fc_result_of(FC_LIBRARY_MISSING, FC_CALL_LOAD);
}
