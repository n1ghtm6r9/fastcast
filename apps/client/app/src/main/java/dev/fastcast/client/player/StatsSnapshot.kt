package dev.fastcast.client.player

import dev.fastcast.client.decoder.OutputColor

class StatsSnapshot(
    val width: Int,
    val height: Int,
    val framesPerSecond: Double,
    val megabitsPerSecond: Double,
    val decoderName: String,
    val droppedFrames: Long,
    val mime: String,
    val color: OutputColor,
    val receiveBufferBytes: Int = 0,
    val delayMs: Double? = null,
    val tunneled: Boolean = false,
    val socketDrops: Int? = null,
    val inboxDrops: Int? = null,
    val linkPauseMs: Int? = null,
)
