package dev.fastcast.client.decoder

import kotlin.math.roundToLong

object DelayTrendText {

    private const val NONE = "none"
    private const val MILLISECONDS = " ms"
    private const val MICROS_PER_MILLISECOND = 1_000.0

    fun of(trend: MedianTrend): String {
        val medians = trend.medians()
        if (medians.isEmpty()) return NONE
        return medians.joinToString(" ", postfix = MILLISECONDS) { (it / MICROS_PER_MILLISECOND).roundToLong().toString() }
    }
}
