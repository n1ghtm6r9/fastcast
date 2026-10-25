#include "fc_internal.h"

NV_ENC_REGISTERED_PTR fc_find_texture(const fc_encoder* encoder, const void* d3d11_texture)
{
    uint32_t index;

    for (index = 0; index < encoder->texture_count; index++) {
        if (encoder->textures[index].resource == d3d11_texture) {
            return encoder->textures[index].registered;
        }
    }

    return NULL;
}
