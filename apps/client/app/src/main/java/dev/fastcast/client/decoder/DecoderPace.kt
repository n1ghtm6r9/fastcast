package dev.fastcast.client.decoder

enum class DecoderPace {
    REALTIME,
    UNDECLARED,
    ;

    val priority: Int
        get() = if (this == REALTIME) REALTIME_PRIORITY else BEST_EFFORT_PRIORITY

    fun frameRate(fps: Int): Int? = fps.takeUnless { this == UNDECLARED }

    fun operatingRate(fps: Int): Int? = fps.takeUnless { this == UNDECLARED }

    private companion object {
        const val REALTIME_PRIORITY = 0
        const val BEST_EFFORT_PRIORITY = 1
    }
}
