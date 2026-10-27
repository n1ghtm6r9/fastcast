package dev.fastcast.client.decoder

import android.media.MediaCodecInfo
import android.media.MediaCodecInfo.CodecCapabilities
import android.media.MediaCodecInfo.VideoCapabilities
import android.media.MediaCodecInfo.VideoCapabilities.PerformancePoint
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

    fun selectTunneled(mime: String, probe: MediaFormat): DecoderChoice? {
        val best = decodersFor(mime)
            .filter { supportsTunnel(it, mime) }
            .minByOrNull { penaltyOf(it, mime, probe) }
            ?: return null
        return DecoderChoice(best.name, supportsLowLatency(best, mime))
    }

    fun canDecodeInHardware(mime: String, width: Int, height: Int, fps: Int): Boolean =
        decodersFor(mime).any { sustainsInHardware(it, mime, width, height, fps) }

    fun canDecodeTenBitInHardware(mime: String, width: Int, height: Int, fps: Int): Boolean =
        decodersFor(mime).any { sustainsInHardware(it, mime, width, height, fps) && hasTenBitProfile(it, mime) }

    fun canTunnelInHardware(mime: String, width: Int, height: Int, fps: Int): Boolean =
        decodersFor(mime).any { !isSoftware(it) && supportsTunnel(it, mime) && declaresSizeAndRate(it, mime, width, height, fps) }

    fun allowsTunnelInHardware(mime: String, width: Int, height: Int, fps: Int): Boolean =
        decodersFor(mime).any { !isSoftware(it) && supportsTunnel(it, mime) && allowsSizeAndRate(it, mime, width, height, fps) }

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

    private fun sustainsInHardware(info: MediaCodecInfo, mime: String, width: Int, height: Int, fps: Int): Boolean =
        !isSoftware(info) && supportsSizeAndRate(info, mime, width, height, fps)

    private fun hasTenBitProfile(info: MediaCodecInfo, mime: String): Boolean =
        withCapabilities(info, mime, false) { capabilities ->
            TenBitProfiles.includes(mime, capabilities.profileLevels.map { it.profile })
        }

    private fun supportsSizeAndRate(info: MediaCodecInfo, mime: String, width: Int, height: Int, fps: Int): Boolean =
        withCapabilities(info, mime, false) { capabilities ->
            capabilities.videoCapabilities?.let { sustains(it, width, height, fps) } == true
        }

    private fun declaresSizeAndRate(info: MediaCodecInfo, mime: String, width: Int, height: Int, fps: Int): Boolean =
        withCapabilities(info, mime, false) { capabilities ->
            capabilities.videoCapabilities?.areSizeAndRateSupported(width, height, fps.toDouble()) == true
        }

    private fun allowsSizeAndRate(info: MediaCodecInfo, mime: String, width: Int, height: Int, fps: Int): Boolean =
        withCapabilities(info, mime, false) { capabilities ->
            capabilities.videoCapabilities?.let { it.isSizeSupported(width, height) && it.supportedFrameRates.upper >= fps } == true
        }

    private fun sustains(video: VideoCapabilities, width: Int, height: Int, fps: Int): Boolean =
        performancePointsCover(video, width, height, fps) ?: video.areSizeAndRateSupported(width, height, fps.toDouble())

    private fun performancePointsCover(video: VideoCapabilities, width: Int, height: Int, fps: Int): Boolean? {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return null
        val points = video.supportedPerformancePoints
        if (points.isNullOrEmpty()) return null
        val required = PerformancePoint(width, height, fps)
        return points.any { it.covers(required) }
    }

    private fun supportsLowLatency(info: MediaCodecInfo, mime: String): Boolean =
        Build.VERSION.SDK_INT >= Build.VERSION_CODES.R &&
            withCapabilities(info, mime, false) { it.isFeatureSupported(CodecCapabilities.FEATURE_LowLatency) }

    private fun supportsTunnel(info: MediaCodecInfo, mime: String): Boolean =
        withCapabilities(info, mime, false) { it.isFeatureSupported(CodecCapabilities.FEATURE_TunneledPlayback) }

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
