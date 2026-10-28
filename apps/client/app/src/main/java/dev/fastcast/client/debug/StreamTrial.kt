package dev.fastcast.client.debug

import dev.fastcast.client.decoder.DecoderSetup

class StreamTrial(
    val step: String,
    val width: Int,
    val height: Int,
    val fps: Int,
    val codec: String,
    val tenBit: Boolean,
    val tunneled: Boolean,
    val setup: DecoderSetup,
    val showsText: Boolean,
    val progress: DebugStep,
)
