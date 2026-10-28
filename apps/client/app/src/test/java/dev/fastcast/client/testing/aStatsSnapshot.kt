package dev.fastcast.client.testing

import dev.fastcast.client.decoder.OutputColor
import dev.fastcast.client.player.StatsSnapshot

fun aStatsSnapshot(
    width: Int = 3840,
    height: Int = 2160,
    framesPerSecond: Double = 59.93,
    megabitsPerSecond: Double = 31.24,
    decoderName: String = "c2.mtk.hevc.decoder",
    droppedFrames: Long = 0L,
    mime: String = "video/hevc",
    color: OutputColor = OutputColor(standard = 1, range = 2, transfer = 3),
    receiveBufferBytes: Int = 8_388_608,
    delayMs: Double? = null,
    tunneled: Boolean = false,
    socketDrops: Int? = null,
    inboxDrops: Int? = null,
    linkPauseMs: Int? = null,
): StatsSnapshot = StatsSnapshot(
    width = width,
    height = height,
    framesPerSecond = framesPerSecond,
    megabitsPerSecond = megabitsPerSecond,
    decoderName = decoderName,
    droppedFrames = droppedFrames,
    mime = mime,
    color = color,
    receiveBufferBytes = receiveBufferBytes,
    delayMs = delayMs,
    tunneled = tunneled,
    socketDrops = socketDrops,
    inboxDrops = inboxDrops,
    linkPauseMs = linkPauseMs,
)
