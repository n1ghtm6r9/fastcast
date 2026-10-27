package dev.fastcast.client.decoder

import java.util.concurrent.atomic.AtomicLong

class DecoderStats {

    val framesQueued = AtomicLong()
    val framesDecoded = AtomicLong()
    val framesRendered = AtomicLong()
    val framesDropped = AtomicLong()
    val presented = PresentTimes(PRESENTED_CAPACITY)
    val framesPresented = AtomicLong()
    val delays = RenderDelays(DELAY_CAPACITY)

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
    }

    fun framePresented(ptsUs: Long, nanos: Long) {
        framesPresented.incrementAndGet()
        presented.add(nanos)
        delays.rendered(ptsUs, nanos)
    }

    private companion object {
        const val PRESENTED_CAPACITY = 4_096
        const val DELAY_CAPACITY = 600
    }
}
