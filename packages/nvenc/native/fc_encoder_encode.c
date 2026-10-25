#include "fc_internal.h"

fc_result fc_encoder_encode(fc_encoder* encoder, void* d3d11_texture, uint32_t force_keyframe, fc_packet* packet)
{
    NV_ENC_MAP_INPUT_RESOURCE mapped;
    NV_ENC_REGISTERED_PTR registered;
    fc_result result;
    NVENCSTATUS status;

    if (encoder == NULL || d3d11_texture == NULL || packet == NULL) {
        return fc_result_of(FC_INVALID_SETTINGS, FC_CALL_NONE);
    }
    memset(packet, 0, sizeof(*packet));

    result = fc_register_texture(encoder, d3d11_texture, &registered);
    if (result.status != 0) {
        return result;
    }

    memset(&mapped, 0, sizeof(mapped));
    mapped.version = NV_ENC_MAP_INPUT_RESOURCE_VER;
    mapped.registeredResource = registered;
    status = encoder->api.nvEncMapInputResource(encoder->session, &mapped);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_MAP);
    }

    result = fc_encode_picture(encoder, &mapped, force_keyframe);
    if (result.status == 0) {
        result = fc_collect_packet(encoder, packet);
    }

    status = encoder->api.nvEncUnmapInputResource(encoder->session, mapped.mappedResource);
    if (status != NV_ENC_SUCCESS && result.status == 0) {
        result = fc_result_of((int32_t)status, FC_CALL_UNMAP);
    }

    return result;
}
