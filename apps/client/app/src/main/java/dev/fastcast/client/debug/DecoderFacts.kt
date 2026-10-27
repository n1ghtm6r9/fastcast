package dev.fastcast.client.debug

import android.media.MediaCodecInfo
import android.media.MediaCodecInfo.CodecCapabilities
import android.media.MediaCodecInfo.VideoCapabilities
import android.media.MediaCodecList
import android.os.Build

object DecoderFacts {

    private const val STEP = "decoders"
    private const val VIDEO = "video/"
    private const val WIDTH_4K = 3840
    private const val HEIGHT_4K = 2160
    private const val BITS_PER_MEGABIT = 1_000_000
    private const val POINT_PREFIX = "PerformancePoint"
    private val PROBED_RATES = listOf(60, 120, 144)
    private val FEATURES = listOf(
        "low-latency",
        "tunneled-playback",
        "adaptive-playback",
        "secure-playback",
        "partial-frame",
        "frame-parsing",
        "multiple-frames",
        "dynamic-timestamp",
    )

    fun of(): FactSection = FactSection(
        STEP,
        MediaCodecList(MediaCodecList.ALL_CODECS).codecInfos
            .filter { info -> !info.isEncoder && videoTypesOf(info).isNotEmpty() }
            .flatMap(::factsOf),
    )

    private fun videoTypesOf(info: MediaCodecInfo): List<String> =
        info.supportedTypes.filter { it.startsWith(VIDEO, ignoreCase = true) }

    private fun factsOf(info: MediaCodecInfo): List<String> =
        listOf(headerOf(info)) + videoTypesOf(info).map { typeFactOf(info, it) }

    private fun headerOf(info: MediaCodecInfo): String {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return "decoder ${info.name}"
        return "decoder ${info.name} (${info.canonicalName}) hardware ${info.isHardwareAccelerated} " +
            "software ${info.isSoftwareOnly} vendor ${info.isVendor} alias ${info.isAlias}"
    }

    private fun typeFactOf(info: MediaCodecInfo, type: String): String = try {
        val capabilities = info.getCapabilitiesForType(type)
        (listOf("${info.name} $type") + videoFactsOf(capabilities.videoCapabilities) + codecFactsOf(capabilities))
            .joinToString(", ")
    } catch (e: Exception) {
        "${info.name} $type: ${e.javaClass.simpleName} ${e.message}"
    }

    private fun videoFactsOf(video: VideoCapabilities?): List<String> {
        if (video == null) return emptyList()
        return listOfNotNull(
            "max ${video.supportedWidths.upper}x${video.supportedHeights.upper}",
            "rates ${video.supportedFrameRates}",
            "bitrate ${video.bitrateRange.upper / BITS_PER_MEGABIT} Mbit/s",
            "4K " + PROBED_RATES.joinToString(" ") { "$it:${video.areSizeAndRateSupported(WIDTH_4K, HEIGHT_4K, it.toDouble())}" },
            runCatching { video.getAchievableFrameRatesFor(WIDTH_4K, HEIGHT_4K) }.getOrNull()?.let { "achievable 4K $it" },
            pointsOf(video),
        )
    }

    private fun pointsOf(video: VideoCapabilities): String? {
        if (Build.VERSION.SDK_INT < Build.VERSION_CODES.Q) return null
        val points = video.supportedPerformancePoints ?: return null
        return "points " + points.joinToString(" ") { it.toString().removePrefix(POINT_PREFIX) }
    }

    private fun codecFactsOf(capabilities: CodecCapabilities): List<String> = listOf(
        "features ${FEATURES.filter(capabilities::isFeatureSupported).joinToString(",")}",
        "instances ${capabilities.maxSupportedInstances}",
        "profiles " + capabilities.profileLevels
            .groupBy({ it.profile }, { it.level })
            .entries
            .joinToString(",") { (profile, levels) -> "$profile:${levels.max()}" },
    )
}
