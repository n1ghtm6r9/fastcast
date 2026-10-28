package dev.fastcast.client.decoder

enum class DecoderTweak {
    NONE,
    LOW_LATENCY,
    LOW_LATENCY_GAME_LIVE,
    MEDIATEK_GAME,
    MEDIATEK_GAME_LIVE,
    MEDIATEK_REAL_TIME,
    ;

    val lowLatency: Boolean
        get() = this == LOW_LATENCY || this == LOW_LATENCY_GAME_LIVE

    val usesStandardLowLatency: Boolean
        get() = !playsAsGame

    fun operatingRate(fps: Int): Int? = fps.takeIf { playsAsGame }

    fun vendorValues(): Map<String, Number> = when (this) {
        MEDIATEK_GAME, MEDIATEK_GAME_LIVE -> GAME_VALUES
        MEDIATEK_REAL_TIME -> REAL_TIME_VALUES
        NONE, LOW_LATENCY, LOW_LATENCY_GAME_LIVE -> emptyMap()
    }

    fun liveVendorValues(): Map<String, Number> = when (this) {
        MEDIATEK_GAME_LIVE, LOW_LATENCY_GAME_LIVE -> GAME_VALUES
        MEDIATEK_REAL_TIME -> REAL_TIME_VALUES
        NONE, LOW_LATENCY, MEDIATEK_GAME -> emptyMap()
    }

    private val playsAsGame: Boolean
        get() = this == MEDIATEK_GAME || this == MEDIATEK_GAME_LIVE

    private companion object {
        const val ENABLED = 1
        val GAME_VALUES = mapOf(MediaTekParameters.GAME_MODE to ENABLED, MediaTekParameters.LOW_LATENCY_MODE to ENABLED)
        val REAL_TIME_VALUES = mapOf(MediaTekParameters.IVIEW_FORCE_REAL_TIME to ENABLED)
    }
}
