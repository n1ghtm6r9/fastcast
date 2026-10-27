package dev.fastcast.client.decoder

enum class DecoderTweak {
    NONE,
    LOW_LATENCY,
    MEDIATEK_GAME,
    FORCED_DISPLAY_RATE,
    FORCED_STREAM_RATE,
    RAISED_RATE_LIMIT,
    ;

    val lowLatency: Boolean
        get() = this == LOW_LATENCY

    val usesStandardLowLatency: Boolean
        get() = this != MEDIATEK_GAME

    fun operatingRate(fps: Int): Int? = fps.takeIf { this == MEDIATEK_GAME }

    fun vendorValues(fps: Int): Map<String, Number> = when (this) {
        MEDIATEK_GAME -> mapOf(MediaTekParameters.GAME_MODE to ENABLED, MediaTekParameters.LOW_LATENCY_MODE to ENABLED)
        FORCED_DISPLAY_RATE -> mapOf(MediaTekParameters.FORCE_FRAME_RATE to DISPLAY_RATE)
        FORCED_STREAM_RATE -> mapOf(MediaTekParameters.FORCE_FRAME_RATE to fps)
        RAISED_RATE_LIMIT -> mapOf(MediaTekParameters.MAX_FRAME_RATE to fps.toFloat())
        NONE, LOW_LATENCY -> emptyMap()
    }

    private companion object {
        const val ENABLED = 1
        const val DISPLAY_RATE = 60
    }
}
