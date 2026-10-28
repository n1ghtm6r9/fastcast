package dev.fastcast.client.decoder

import kotlin.math.roundToInt

class FrameLateness(private val capacity: Int) {

    private val latenessUs = ArrayDeque<Long>()

    @Synchronized
    fun add(lateUs: Long) {
        latenessUs.addLast(lateUs)
        if (latenessUs.size > capacity) latenessUs.removeFirst()
    }

    @Synchronized
    fun percentileMs(fraction: Double): Double? {
        if (latenessUs.isEmpty()) return null
        val sorted = latenessUs.sorted()
        return sorted[((sorted.size - 1) * fraction).roundToInt()] / MICROS_PER_MILLISECOND
    }

    private companion object {
        const val MICROS_PER_MILLISECOND = 1_000.0
    }
}
