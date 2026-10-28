package dev.fastcast.client.decoder

import java.util.concurrent.atomic.AtomicLong

class DecoderStats {

    val framesQueued = AtomicLong()
    val framesDecoded = AtomicLong()
    val framesRendered = AtomicLong()
    val framesDropped = AtomicLong()
    val codecStarts = AtomicLong()
    val presented = PresentTimes(PRESENTED_CAPACITY)
    val framesPresented = AtomicLong()
    val delays = RenderDelays(DELAY_CAPACITY)
    val decodeDelays = RenderDelays(DELAY_CAPACITY)
    val lateness = FrameLateness(DELAY_CAPACITY)
    val delayTrend = MedianTrend(TREND_GROUP_FRAMES, TREND_GROUPS)
    val latenessTrend = MedianTrend(TREND_GROUP_FRAMES, TREND_GROUPS)

    @Volatile
    var decoderName: String = ""

    @Volatile
    var mime: String = ""

    @Volatile
    var outputColor: OutputColor = OutputColor.NONE

    @Volatile
    var tunnelFrameReady: Boolean = false

    @Volatile
    var codecParameters: CodecParameters? = null

    fun frameQueued(ptsUs: Long, nanos: Long) {
        framesQueued.incrementAndGet()
        delays.queued(ptsUs, nanos)
        decodeDelays.queued(ptsUs, nanos)
    }

    fun frameDecoded(ptsUs: Long, nanos: Long) {
        framesDecoded.incrementAndGet()
        decodeDelays.rendered(ptsUs, nanos)
    }

    fun framePresented(ptsUs: Long, nanos: Long) {
        framesPresented.incrementAndGet()
        presented.add(nanos)
        delays.rendered(ptsUs, nanos)?.let { delayTrend.add(it / NANOS_PER_MICROSECOND) }
    }

    fun frameLate(lateUs: Long) {
        lateness.add(lateUs)
        latenessTrend.add(lateUs)
    }

    private companion object {
        const val PRESENTED_CAPACITY = 4_096
        const val DELAY_CAPACITY = 600
        const val TREND_GROUP_FRAMES = 120
        const val TREND_GROUPS = 12
        const val NANOS_PER_MICROSECOND = 1_000L
    }
}
