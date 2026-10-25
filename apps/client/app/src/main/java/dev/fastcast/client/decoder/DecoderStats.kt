package dev.fastcast.client.decoder

import java.util.concurrent.atomic.AtomicLong

class DecoderStats {

    val framesRendered = AtomicLong()
    val framesDropped = AtomicLong()

    @Volatile
    var decoderName: String = ""
}
