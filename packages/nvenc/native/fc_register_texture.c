#include "fc_internal.h"

fc_result fc_register_texture(fc_encoder* encoder, void* d3d11_texture, NV_ENC_REGISTERED_PTR* registered)
{
    NV_ENC_REGISTER_RESOURCE resource;
    fc_texture* slot;
    NVENCSTATUS status;

    *registered = fc_find_texture(encoder, d3d11_texture);
    if (*registered != NULL) {
        return fc_result_of(0, FC_CALL_NONE);
    }
    if (encoder->texture_count == FC_MAX_TEXTURES) {
        return fc_result_of(FC_TEXTURE_LIMIT, FC_CALL_NONE);
    }

    memset(&resource, 0, sizeof(resource));
    resource.version = NV_ENC_REGISTER_RESOURCE_VER;
    resource.resourceType = NV_ENC_INPUT_RESOURCE_TYPE_DIRECTX;
    resource.width = encoder->settings.width;
    resource.height = encoder->settings.height;
    resource.pitch = 0;
    resource.subResourceIndex = 0;
    resource.resourceToRegister = d3d11_texture;
    resource.bufferFormat = fc_buffer_format(encoder->settings.bit_depth);
    resource.bufferUsage = NV_ENC_INPUT_IMAGE;
    status = encoder->api.nvEncRegisterResource(encoder->session, &resource);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_REGISTER);
    }

    IUnknown_AddRef((IUnknown*)d3d11_texture);
    slot = &encoder->textures[encoder->texture_count];
    slot->resource = d3d11_texture;
    slot->registered = resource.registeredResource;
    encoder->texture_count += 1;
    *registered = resource.registeredResource;
    return fc_result_of(0, FC_CALL_NONE);
}
