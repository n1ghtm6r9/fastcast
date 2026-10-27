package dev.fastcast.client.debug

import dev.fastcast.client.decoder.DecoderPace
import dev.fastcast.client.decoder.DecoderTweak

class StreamTrial(
    val step: String,
    val width: Int,
    val height: Int,
    val fps: Int,
    val tunneled: Boolean,
    val pace: DecoderPace,
    val tweak: DecoderTweak,
    val progress: DebugStep,
)
