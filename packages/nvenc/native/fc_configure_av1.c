#include "fc_internal.h"

void fc_configure_av1(NV_ENC_CONFIG_AV1* av1, uint32_t bit_depth)
{
    NV_ENC_BIT_DEPTH depth = bit_depth == 10u ? NV_ENC_BIT_DEPTH_10 : NV_ENC_BIT_DEPTH_8;

    av1->idrPeriod = NVENC_INFINITE_GOPLENGTH;
    av1->repeatSeqHdr = 1;
    av1->inputBitDepth = depth;
    av1->outputBitDepth = depth;
    av1->colorPrimaries = NV_ENC_VUI_COLOR_PRIMARIES_BT709;
    av1->transferCharacteristics = NV_ENC_VUI_TRANSFER_CHARACTERISTIC_BT709;
    av1->matrixCoefficients = NV_ENC_VUI_MATRIX_COEFFS_BT709;
    av1->colorRange = 0;
    av1->chromaSamplePosition = 1;
}
