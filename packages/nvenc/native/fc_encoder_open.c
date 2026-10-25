#include "fc_internal.h"

fc_result fc_encoder_open(void* d3d11_device, const fc_settings* settings, fc_encoder** encoder)
{
    fc_encoder* created;
    fc_result result;

    if (encoder == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }
    *encoder = NULL;

    result = fc_validate_settings(d3d11_device, settings);
    if (result.status != 0) {
        return result;
    }

    created = (fc_encoder*)calloc(1, sizeof(fc_encoder));
    if (created == NULL) {
        return fc_result_of(FC_OUT_OF_MEMORY, FC_CALL_NONE);
    }
    created->settings = *settings;
    IUnknown_AddRef((IUnknown*)d3d11_device);
    created->device = d3d11_device;

    result = fc_load_api(created);
    if (result.status == 0) {
        result = fc_open_session(created);
    }
    if (result.status == 0) {
        result = fc_check_codec(created);
    }
    if (result.status == 0) {
        result = fc_check_bit_depth(created);
    }
    if (result.status == 0) {
        result = fc_configure(created);
    }
    if (result.status == 0) {
        result = fc_initialize(created);
    }
    if (result.status == 0) {
        result = fc_create_bitstream(created);
    }
    if (result.status != 0) {
        fc_encoder_close(created);
        return result;
    }

    *encoder = created;
    return result;
}
