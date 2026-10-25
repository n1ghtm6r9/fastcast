#pragma once

#define COBJMACROS

#include <stdlib.h>
#include <string.h>
#include <windows.h>
#include <unknwn.h>

#include "fastcast_nvenc.h"
#include "nvEncodeAPI.h"

#define FC_MIN_DIMENSION 2u
#define FC_MAX_DIMENSION 8192u
#define FC_MAX_FPS 240u
#define FC_MAX_BITRATE_KBPS 800000u
#define FC_MIN_PRESET 1u
#define FC_MAX_PRESET 7u
#define FC_REQUIRED_API_VERSION ((((uint32_t)NVENCAPI_MAJOR_VERSION) << 4) | ((uint32_t)NVENCAPI_MINOR_VERSION))

typedef NVENCSTATUS(NVENCAPI* fc_get_max_version_fn)(uint32_t* version);
typedef NVENCSTATUS(NVENCAPI* fc_create_instance_fn)(NV_ENCODE_API_FUNCTION_LIST* functions);

struct fc_encoder {
    fc_settings settings;
    HMODULE library;
    void* device;
    void* session;
    NV_ENCODE_API_FUNCTION_LIST api;
    NV_ENC_CONFIG config;
    NV_ENC_INITIALIZE_PARAMS init;
    NV_ENC_OUTPUT_PTR bitstream;
};

fc_result fc_result_of(int32_t status, uint32_t call);
fc_result fc_validate_settings(const void* owner, const fc_settings* settings);
fc_result fc_load_api(fc_encoder* encoder);
fc_result fc_open_session(fc_encoder* encoder);
fc_result fc_check_codec(fc_encoder* encoder);
fc_result fc_check_bit_depth(fc_encoder* encoder);
fc_result fc_configure(fc_encoder* encoder);
fc_result fc_initialize(fc_encoder* encoder);
fc_result fc_create_bitstream(fc_encoder* encoder);
GUID fc_codec_guid(uint32_t codec);
GUID fc_preset_guid(uint32_t preset);
GUID fc_profile_guid(uint32_t codec, uint32_t bit_depth);
uint32_t fc_vbv_size(uint32_t bitrate_kbps, uint32_t fps);
void fc_set_vui(NV_ENC_CONFIG_H264_VUI_PARAMETERS* vui);
void fc_configure_h264(NV_ENC_CONFIG_H264* h264, uint32_t bit_depth);
void fc_configure_hevc(NV_ENC_CONFIG_HEVC* hevc, uint32_t bit_depth);
