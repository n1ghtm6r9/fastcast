#include "fc_internal.h"

void fc_release_textures(fc_encoder* encoder)
{
    uint32_t index;

    for (index = 0; index < encoder->texture_count; index++) {
        encoder->api.nvEncUnregisterResource(encoder->session, encoder->textures[index].registered);
        IUnknown_Release((IUnknown*)encoder->textures[index].resource);
    }
    encoder->texture_count = 0;
}
