#include "fc_internal.h"

fc_result fc_create_bitstream(fc_encoder* encoder)
{
    NV_ENC_CREATE_BITSTREAM_BUFFER create;
    NVENCSTATUS status;

    memset(&create, 0, sizeof(create));
    create.version = NV_ENC_CREATE_BITSTREAM_BUFFER_VER;
    status = encoder->api.nvEncCreateBitstreamBuffer(encoder->session, &create);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_CREATE_BITSTREAM);
    }

    encoder->bitstream = create.bitstreamBuffer;
    return fc_result_of(0, FC_CALL_NONE);
}
