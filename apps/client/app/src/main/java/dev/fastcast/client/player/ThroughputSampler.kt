package dev.fastcast.client.player

class ThroughputSampler(startMs: Long) {

    private var lastTimeMs = startMs
    private var lastBytes = 0L
    private var lastPresented = 0L
    private var lastRendered = 0L

    fun sample(nowMs: Long, totalBytes: Long, presentedFrames: Long, renderedFrames: Long): Throughput {
        val elapsedMs = nowMs - lastTimeMs
        val bytes = totalBytes - lastBytes
        val presented = presentedFrames - lastPresented
        val frames = if (presented > 0L) presented else renderedFrames - lastRendered
        lastTimeMs = nowMs
        lastBytes = totalBytes
        lastPresented = presentedFrames
        lastRendered = renderedFrames
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
