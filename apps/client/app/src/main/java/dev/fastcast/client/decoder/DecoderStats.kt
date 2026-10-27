package dev.fastcast.client.decoder

import java.util.concurrent.atomic.AtomicLong

class DecoderStats {

    val framesDecoded = AtomicLong()
    val framesRendered = AtomicLong()
    val framesDropped = AtomicLong()
    val presented = PresentTimes(PRESENTED_CAPACITY)

    @Volatile
    var decoderName: String = ""

    @Volatile
    var mime: String = ""

    @Volatile
    var outputColor: OutputColor = OutputColor.NONE

    private companion object {
        const val PRESENTED_CAPACITY = 4_096
    }
}
