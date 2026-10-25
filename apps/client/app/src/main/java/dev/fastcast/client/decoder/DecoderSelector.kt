package dev.fastcast.client.decoder

import android.media.MediaCodecInfo
import android.media.MediaCodecInfo.CodecCapabilities
import android.media.MediaCodecList
import android.media.MediaFormat
import android.os.Build

object DecoderSelector {

    private val SOFTWARE_PREFIXES = listOf("OMX.google.", "c2.android.")
    private const val SECURE_SUFFIX = ".secure"
    private const val SOFTWARE_PENALTY = 2
    private const val UNSUPPORTED_PENALTY = 1

    fun select(mime: String, probe: MediaFormat): DecoderChoice? {
        val best = decodersFor(mime).minByOrNull { penaltyOf(it, mime, probe) } ?: return null
        return DecoderChoice(best.name, supportsLowLatency(best, mime))
    }

    fun canDecodeInHardware(mime: String, width: Int, height: Int, fps: Int): Boolean =
        decodersFor(mime).any { !isSoftware(it) && supportsSizeAndRate(it, mime, width, height, fps) }

    private fun decodersFor(mime: String): List<MediaCodecInfo> =
        MediaCodecList(MediaCodecList.REGULAR_CODECS).codecInfos.filter { isDecoderFor(it, mime) }

    private fun isDecoderFor(info: MediaCodecInfo, mime: String): Boolean =
        !info.isEncoder && !info.name.endsWith(SECURE_SUFFIX) && typeFor(info, mime) != null

    private fun typeFor(info: MediaCodecInfo, mime: String): String? =
        info.supportedTypes.firstOrNull { it.equals(mime, ignoreCase = true) }

    private fun penaltyOf(info: MediaCodecInfo, mime: String, probe: MediaFormat): Int {
        val softwarePenalty = if (isSoftware(info)) SOFTWARE_PENALTY else 0
        val unsupportedPenalty = if (supportsFormat(info, mime, probe)) 0 else UNSUPPORTED_PENALTY
        return softwarePenalty + unsupportedPenalty
    }

    private fun isSoftware(info: MediaCodecInfo): Boolean =
        SOFTWARE_PREFIXES.any { info.name.startsWith(it, ignoreCase = true) } ||
            (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q && info.isSoftwareOnly)

    private fun supportsFormat(info: MediaCodecInfo, mime: String, probe: MediaFormat): Boolean =
        withCapabilities(info, mime, false) { it.isFormatSupported(probe) }

    private fun supportsSizeAndRate(info: MediaCodecInfo, mime: String, width: Int, height: Int, fps: Int): Boolean =
        withCapabilities(info, mime, false) {
            it.videoCapabilities?.areSizeAndRateSupported(width, height, fps.toDouble()) == true
        }

    private fun supportsLowLatency(info: MediaCodecInfo, mime: String): Boolean =
        Build.VERSION.SDK_INT >= Build.VERSION_CODES.R &&
            withCapabilities(info, mime, false) { it.isFeatureSupported(CodecCapabilities.FEATURE_LowLatency) }

    private fun <T> withCapabilities(
        info: MediaCodecInfo,
        mime: String,
        fallback: T,
        block: (CodecCapabilities) -> T,
    ): T = try {
        block(info.getCapabilitiesForType(typeFor(info, mime) ?: mime))
    } catch (e: IllegalArgumentException) {
        fallback
    }
}
