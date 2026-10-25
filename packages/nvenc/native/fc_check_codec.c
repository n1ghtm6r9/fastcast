#include "fc_internal.h"

fc_result fc_check_codec(fc_encoder* encoder)
{
    GUID wanted = fc_codec_guid(encoder->settings.codec);
    GUID* guids;
    uint32_t count = 0;
    uint32_t returned = 0;
    uint32_t index;
    int found = 0;
    NVENCSTATUS status;

    status = encoder->api.nvEncGetEncodeGUIDCount(encoder->session, &count);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_GET_GUIDS);
    }

    guids = (GUID*)calloc(count > 0u ? count : 1u, sizeof(GUID));
    if (guids == NULL) {
        return fc_result_of(FC_OUT_OF_MEMORY, FC_CALL_GET_GUIDS);
    }

    status = encoder->api.nvEncGetEncodeGUIDs(encoder->session, guids, count, &returned);
    if (status != NV_ENC_SUCCESS) {
        free(guids);
        return fc_result_of((int32_t)status, FC_CALL_GET_GUIDS);
    }

    for (index = 0; index < returned && !found; index++) {
        found = memcmp(&guids[index], &wanted, sizeof(GUID)) == 0;
    }
    free(guids);

    if (!found) {
        return fc_result_of(FC_CODEC_UNSUPPORTED, FC_CALL_GET_GUIDS);
    }
    return fc_result_of(0, FC_CALL_NONE);
}
