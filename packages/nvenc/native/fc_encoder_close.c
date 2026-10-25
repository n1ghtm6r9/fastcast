#include "fc_internal.h"

void fc_encoder_close(fc_encoder* encoder)
{
    if (encoder == NULL) {
        return;
    }

    if (encoder->session != NULL) {
        if (encoder->bitstream != NULL) {
            encoder->api.nvEncDestroyBitstreamBuffer(encoder->session, encoder->bitstream);
        }
        encoder->api.nvEncDestroyEncoder(encoder->session);
    }
    if (encoder->device != NULL) {
        IUnknown_Release((IUnknown*)encoder->device);
    }
    if (encoder->library != NULL) {
        FreeLibrary(encoder->library);
    }
    free(encoder);
}
