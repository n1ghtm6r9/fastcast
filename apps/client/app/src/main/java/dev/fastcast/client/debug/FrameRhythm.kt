package dev.fastcast.client.debug

import java.util.Locale
import kotlin.math.roundToInt

class FrameRhythm private constructor(
    val frames: Int,
    val seconds: Double,
    val medianMs: Double,
    val commonIntervals: List<Pair<Int, Int>>,
) {

    val perSecond: Double
        get() = if (seconds > 0.0) (frames - 1) / seconds else 0.0

    fun describe(label: String): String {
        if (frames < 2) return "$label: $frames frames"
        val intervals = commonIntervals.joinToString(" ") { (ms, count) -> "$ms:$count" }
        return DESCRIPTION.format(Locale.ROOT, label, frames, seconds, perSecond, medianMs, intervals)
    }

    companion object {
        private const val DESCRIPTION = "%s: %d frames in %.2f s = %.1f/s, median %.2f ms, ms %s"
        private const val NANOS_PER_SECOND = 1_000_000_000.0
        private const val NANOS_PER_MILLISECOND = 1_000_000.0
        private const val LISTED_INTERVALS = 6

        fun of(times: LongArray): FrameRhythm {
            val sorted = times.sorted()
            val intervalsMs = sorted.zipWithNext { earlier, later -> (later - earlier) / NANOS_PER_MILLISECOND }
            val seconds = if (sorted.size < 2) 0.0 else (sorted.last() - sorted.first()) / NANOS_PER_SECOND
            return FrameRhythm(sorted.size, seconds, medianOf(intervalsMs), mostCommonOf(intervalsMs))
        }

        private fun medianOf(values: List<Double>): Double {
            if (values.isEmpty()) return 0.0
            val sorted = values.sorted()
            val middle = sorted.size / 2
            return if (sorted.size % 2 == 1) sorted[middle] else (sorted[middle - 1] + sorted[middle]) / 2
        }

        private fun mostCommonOf(intervalsMs: List<Double>): List<Pair<Int, Int>> =
            intervalsMs.groupingBy { it.roundToInt() }
                .eachCount()
                .entries
                .sortedWith(compareByDescending<Map.Entry<Int, Int>> { it.value }.thenBy { it.key })
                .take(LISTED_INTERVALS)
                .map { it.key to it.value }
    }
}
