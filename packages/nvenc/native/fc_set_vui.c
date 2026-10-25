#include "fc_internal.h"

void fc_set_vui(NV_ENC_CONFIG_H264_VUI_PARAMETERS* vui)
{
    vui->videoSignalTypePresentFlag = 1;
    vui->videoFormat = NV_ENC_VUI_VIDEO_FORMAT_UNSPECIFIED;
    vui->videoFullRangeFlag = 0;
    vui->colourDescriptionPresentFlag = 1;
    vui->colourPrimaries = NV_ENC_VUI_COLOR_PRIMARIES_BT709;
    vui->transferCharacteristics = NV_ENC_VUI_TRANSFER_CHARACTERISTIC_BT709;
    vui->colourMatrix = NV_ENC_VUI_MATRIX_COEFFS_BT709;
    vui->chromaSampleLocationFlag = 1;
    vui->chromaSampleLocationTop = 0;
    vui->chromaSampleLocationBot = 0;
}
