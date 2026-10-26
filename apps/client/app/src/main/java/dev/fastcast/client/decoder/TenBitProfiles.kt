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

    fun includes(mime: String, profiles: List<Int>): Boolean =
        mime.equals(VideoMime.HEVC, ignoreCase = true) && profiles.any { it in HEVC_MAIN_TEN }
}
