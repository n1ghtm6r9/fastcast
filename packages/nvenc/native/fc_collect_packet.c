#include "fc_internal.h"

fc_result fc_collect_packet(fc_encoder* encoder, fc_packet* packet)
{
    NV_ENC_LOCK_BITSTREAM lock;
    fc_result result;
    NVENCSTATUS status;

    memset(&lock, 0, sizeof(lock));
    lock.version = NV_ENC_LOCK_BITSTREAM_VER;
    lock.outputBitstream = encoder->bitstream;
    status = encoder->api.nvEncLockBitstream(encoder->session, &lock);
    if (status != NV_ENC_SUCCESS) {
        return fc_result_of((int32_t)status, FC_CALL_LOCK);
    }

    result = fc_reserve_output(encoder, lock.bitstreamSizeInBytes);
    if (result.status == 0) {
        memcpy(encoder->output, lock.bitstreamBufferPtr, lock.bitstreamSizeInBytes);
        packet->data = encoder->output;
        packet->size = lock.bitstreamSizeInBytes;
        packet->keyframe = lock.pictureType == NV_ENC_PIC_TYPE_IDR ? 1u : 0u;
    }

    status = encoder->api.nvEncUnlockBitstream(encoder->session, encoder->bitstream);
    if (status != NV_ENC_SUCCESS && result.status == 0) {
        result = fc_result_of((int32_t)status, FC_CALL_UNLOCK);
    }

    return result;
}
