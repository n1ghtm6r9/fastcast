package dev.fastcast.client.decoder

enum class DecoderPace {
    REALTIME,
    BEST_EFFORT,
    FULL_SPEED,
    UNDECLARED,
    ;

    val priority: Int
        get() = if (this == REALTIME) REALTIME_PRIORITY else BEST_EFFORT_PRIORITY

    fun frameRate(fps: Int): Int? = fps.takeUnless { this == UNDECLARED }

    fun operatingRate(fps: Int): Int? = when (this) {
        REALTIME, BEST_EFFORT -> fps
        FULL_SPEED -> HIGHEST_OPERATING_RATE
        UNDECLARED -> null
    }

    companion object {
        private const val REALTIME_PRIORITY = 0
        private const val BEST_EFFORT_PRIORITY = 1
        private const val HIGHEST_OPERATING_RATE = Short.MAX_VALUE.toInt()

        fun of(bestEffort: Boolean): DecoderPace = if (bestEffort) FULL_SPEED else REALTIME
    }
}
