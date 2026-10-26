#pragma once

#include <stdint.h>

typedef struct fc_encoder fc_encoder;

typedef struct fc_settings {
    uint32_t codec;
    uint32_t width;
    uint32_t height;
    uint32_t fps;
    uint32_t bitrate_kbps;
    uint32_t bit_depth;
    uint32_t preset;
} fc_settings;

typedef struct fc_result {
    int32_t status;
    uint32_t call;
} fc_result;

typedef struct fc_packet {
    const uint8_t* data;
    uint32_t size;
    uint32_t keyframe;
} fc_packet;

enum { FC_CODEC_H264 = 1, FC_CODEC_HEVC = 2, FC_CODEC_AV1 = 3 };

enum {
    FC_LIBRARY_MISSING = -1,
    FC_ENTRY_POINT_MISSING = -2,
    FC_DRIVER_TOO_OLD = -3,
    FC_CODEC_UNSUPPORTED = -4,
    FC_INVALID_SETTINGS = -5,
    FC_OUT_OF_MEMORY = -6,
    FC_TEXTURE_LIMIT = -7
};

enum {
    FC_CALL_NONE = 0,
    FC_CALL_LOAD = 1,
    FC_CALL_OPEN_SESSION = 2,
    FC_CALL_GET_GUIDS = 3,
    FC_CALL_GET_CAPS = 4,
    FC_CALL_PRESET_CONFIG = 5,
    FC_CALL_INITIALIZE = 6,
    FC_CALL_CREATE_BITSTREAM = 7,
    FC_CALL_REGISTER = 8,
    FC_CALL_MAP = 9,
    FC_CALL_ENCODE = 10,
    FC_CALL_LOCK = 11,
    FC_CALL_UNLOCK = 12,
    FC_CALL_UNMAP = 13,
    FC_CALL_RECONFIGURE = 14
};

fc_result fc_encoder_open(void* d3d11_device, const fc_settings* settings, fc_encoder** encoder);
fc_result fc_encoder_encode(fc_encoder* encoder, void* d3d11_texture, uint32_t force_keyframe, fc_packet* packet);
fc_result fc_encoder_set_bitrate(fc_encoder* encoder, uint32_t bitrate_kbps);
void fc_encoder_close(fc_encoder* encoder);
