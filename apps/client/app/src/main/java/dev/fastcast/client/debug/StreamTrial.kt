package dev.fastcast.client.debug

import dev.fastcast.client.decoder.DecoderPace

class StreamTrial(
    val step: String,
    val width: Int,
    val height: Int,
    val fps: Int,
    val tunneled: Boolean,
    val pace: DecoderPace,
    val progress: DebugStep,
)
