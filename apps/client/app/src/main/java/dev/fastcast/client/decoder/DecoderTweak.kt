package dev.fastcast.client.decoder

enum class DecoderTweak {
    NONE,
    LOW_LATENCY,
    LOW_LATENCY_FULL_SPEED,
    MEDIATEK_GAME,
    MEDIATEK_GAME_LIVE,
    MEDIATEK_REAL_TIME,
    ;

    val lowLatency: Boolean
        get() = this == LOW_LATENCY || this == LOW_LATENCY_FULL_SPEED

    val usesStandardLowLatency: Boolean
        get() = !playsAsGame

    fun operatingRate(fps: Int): Int? = when {
        playsAsGame -> fps
        this == LOW_LATENCY_FULL_SPEED -> FULL_SPEED
        else -> null
    }

    fun vendorValues(): Map<String, Number> = when (this) {
        MEDIATEK_GAME, MEDIATEK_GAME_LIVE -> GAME_VALUES
        MEDIATEK_REAL_TIME -> REAL_TIME_VALUES
        NONE, LOW_LATENCY, LOW_LATENCY_FULL_SPEED -> emptyMap()
    }

    fun liveVendorValues(): Map<String, Number> = when (this) {
        MEDIATEK_GAME_LIVE -> GAME_VALUES
        MEDIATEK_REAL_TIME -> REAL_TIME_VALUES
        NONE, LOW_LATENCY, LOW_LATENCY_FULL_SPEED, MEDIATEK_GAME -> emptyMap()
    }

    private val playsAsGame: Boolean
        get() = this == MEDIATEK_GAME || this == MEDIATEK_GAME_LIVE

    private companion object {
        const val ENABLED = 1
        const val FULL_SPEED = Short.MAX_VALUE.toInt()
        val GAME_VALUES = mapOf(MediaTekParameters.GAME_MODE to ENABLED, MediaTekParameters.LOW_LATENCY_MODE to ENABLED)
        val REAL_TIME_VALUES = mapOf(MediaTekParameters.IVIEW_FORCE_REAL_TIME to ENABLED)
    }
}
