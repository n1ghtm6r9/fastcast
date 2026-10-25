package dev.fastcast.client.player

class StatsSnapshot(
    val width: Int,
    val height: Int,
    val framesPerSecond: Double,
    val megabitsPerSecond: Double,
    val decoderName: String,
    val droppedFrames: Long,
)
