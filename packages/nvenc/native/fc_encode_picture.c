#include "fc_internal.h"

fc_result fc_encode_picture(fc_encoder* encoder, const NV_ENC_MAP_INPUT_RESOURCE* mapped, uint32_t force_keyframe)
{
    NV_ENC_PIC_PARAMS picture;
    NVENCSTATUS status;

    memset(&picture, 0, sizeof(picture));
    picture.version = NV_ENC_PIC_PARAMS_VER;
    picture.inputWidth = encoder->settings.width;
    picture.inputHeight = encoder->settings.height;
    picture.inputPitch = encoder->settings.width;
    picture.inputBuffer = mapped->mappedResource;
    picture.outputBitstream = encoder->bitstream;
    picture.bufferFmt = mapped->mappedBufferFmt;
    picture.pictureStruct = NV_ENC_PIC_STRUCT_FRAME;
    picture.inputTimeStamp = encoder->timestamp;
    encoder->timestamp += 1;
    if (force_keyframe != 0) {
        picture.encodePicFlags = NV_ENC_PIC_FLAG_FORCEIDR | NV_ENC_PIC_FLAG_OUTPUT_SPSPPS;
    }

    status = encoder->api.nvEncEncodePicture(encoder->session, &picture);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_ENCODE);
    }

    return fc_result_of(0, FC_CALL_NONE);
}
