package dev.fastcast.client.debug

import java.util.Locale

class CounterRates(
    val queuedPerSecond: Double,
    val decodedPerSecond: Double,
    val renderedPerSecond: Double,
    val droppedPerSecond: Double,
    val megabitsPerSecond: Double,
) {

    fun describe(): String = DESCRIPTION.format(
        Locale.ROOT,
        queuedPerSecond,
        decodedPerSecond,
        renderedPerSecond,
        droppedPerSecond,
        megabitsPerSecond,
    )

    companion object {
        private const val DESCRIPTION = "queued %.1f/s, decoded %.1f/s, rendered %.1f/s, dropped %.1f/s, %.1f Mbit/s"
        private const val NANOS_PER_SECOND = 1_000_000_000.0
        private const val BITS_PER_BYTE = 8.0
        private const val BITS_PER_MEGABIT = 1_000_000.0

        fun between(from: DebugCounters, to: DebugCounters): CounterRates {
            val seconds = (to.nanos - from.nanos) / NANOS_PER_SECOND
            if (seconds <= 0.0) return CounterRates(0.0, 0.0, 0.0, 0.0, 0.0)
            return CounterRates(
                queuedPerSecond = (to.queued - from.queued) / seconds,
                decodedPerSecond = (to.decoded - from.decoded) / seconds,
                renderedPerSecond = (to.rendered - from.rendered) / seconds,
                droppedPerSecond = (to.dropped - from.dropped) / seconds,
                megabitsPerSecond = (to.bytes - from.bytes) * BITS_PER_BYTE / BITS_PER_MEGABIT / seconds,
            )
        }
    }
}
