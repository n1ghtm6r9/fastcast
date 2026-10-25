#include "fc_internal.h"

GUID fc_preset_guid(uint32_t preset)
{
    switch (preset) {
    case 1u:
        return NV_ENC_PRESET_P1_GUID;
    case 2u:
        return NV_ENC_PRESET_P2_GUID;
    case 3u:
        return NV_ENC_PRESET_P3_GUID;
    case 5u:
        return NV_ENC_PRESET_P5_GUID;
    case 6u:
        return NV_ENC_PRESET_P6_GUID;
    case 7u:
        return NV_ENC_PRESET_P7_GUID;
    default:
        return NV_ENC_PRESET_P4_GUID;
    }
}
