#include "fc_internal.h"

fc_result fc_load_api(fc_encoder* encoder)
{
    fc_get_max_version_fn get_max_version;
    fc_create_instance_fn create_instance;
    uint32_t supported = 0;
    NVENCSTATUS status;

    encoder->library = LoadLibraryW(L"nvEncodeAPI64.dll");
    if (encoder->library == NULL) {
        return fc_result_of(FC_LIBRARY_MISSING, FC_CALL_LOAD);
    }

    get_max_version = (fc_get_max_version_fn)GetProcAddress(encoder->library, "NvEncodeAPIGetMaxSupportedVersion");
    create_instance = (fc_create_instance_fn)GetProcAddress(encoder->library, "NvEncodeAPICreateInstance");
    if (get_max_version == NULL || create_instance == NULL) {
        return fc_result_of(FC_ENTRY_POINT_MISSING, FC_CALL_LOAD);
    }

    status = get_max_version(&supported);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_LOAD);
    }
    if (supported < FC_REQUIRED_API_VERSION) {
        return fc_result_of(FC_DRIVER_TOO_OLD, FC_CALL_LOAD);
    }

    memset(&encoder->api, 0, sizeof(encoder->api));
    encoder->api.version = NV_ENCODE_API_FUNCTION_LIST_VER;
    status = create_instance(&encoder->api);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_LOAD);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
