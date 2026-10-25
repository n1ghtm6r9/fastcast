#pragma once

#include <windows.h>

#include "fastcast_nvenc.h"
#include "nvEncodeAPI.h"

#define FC_MIN_DIMENSION 2u
#define FC_MAX_DIMENSION 8192u
#define FC_MAX_FPS 240u
#define FC_MAX_BITRATE_KBPS 800000u
#define FC_MIN_PRESET 1u
#define FC_MAX_PRESET 7u

struct fc_encoder {
    fc_settings settings;
};

fc_result fc_result_of(int32_t status, uint32_t call);
fc_result fc_validate_settings(const void* owner, const fc_settings* settings);
