package dev.fastcast.client.decoder

import android.annotation.SuppressLint
import android.media.MediaCodecInfo.CodecProfileLevel

object TenBitProfiles {

    @SuppressLint("InlinedApi")
    private val HEVC_MAIN_TEN = setOf(
        CodecProfileLevel.HEVCProfileMain10,
        CodecProfileLevel.HEVCProfileMain10HDR10,
        CodecProfileLevel.HEVCProfileMain10HDR10Plus,
    )

    @SuppressLint("InlinedApi")
    private val AV1_MAIN_TEN = setOf(
        CodecProfileLevel.AV1ProfileMain10,
        CodecProfileLevel.AV1ProfileMain10HDR10,
        CodecProfileLevel.AV1ProfileMain10HDR10Plus,
    )

    fun includes(mime: String, profiles: List<Int>): Boolean = when {
        mime.equals(VideoMime.HEVC, ignoreCase = true) -> profiles.any { it in HEVC_MAIN_TEN }
        mime.equals(VideoMime.AV1, ignoreCase = true) -> profiles.any { it in AV1_MAIN_TEN }
        else -> false
    }
}
