package dev.fastcast.client.player

class ThroughputSampler(startMs: Long) {

    private var lastTimeMs = startMs
    private var lastBytes = 0L
    private var lastFrames = 0L

    fun sample(nowMs: Long, totalBytes: Long, totalFrames: Long): Throughput {
        val elapsedMs = nowMs - lastTimeMs
        val bytes = totalBytes - lastBytes
        val frames = totalFrames - lastFrames
        lastTimeMs = nowMs
        lastBytes = totalBytes
        lastFrames = totalFrames
        if (elapsedMs <= 0) return Throughput(0.0, 0.0)
        val seconds = elapsedMs / MILLISECONDS_PER_SECOND
        return Throughput(
            framesPerSecond = frames / seconds,
            megabitsPerSecond = bytes * BITS_PER_BYTE / BITS_PER_MEGABIT / seconds,
        )
    }

    private companion object {
        const val MILLISECONDS_PER_SECOND = 1_000.0
        const val BITS_PER_BYTE = 8.0
        const val BITS_PER_MEGABIT = 1_000_000.0
    }
}
