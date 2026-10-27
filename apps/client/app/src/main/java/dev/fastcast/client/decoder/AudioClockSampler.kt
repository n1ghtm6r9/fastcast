package dev.fastcast.client.decoder

class AudioClockSampler(private val startedNanos: Long) {

    @Volatile
    private var first: PlayedAudio? = null

    @Volatile
    private var latest: PlayedAudio? = null

    private var polledAtNanos = 0L

    val movedAfterNanos: Long?
        get() = first?.let { it.atNanos - it.frames * NANOS_PER_SECOND / TunnelClock.SAMPLE_RATE - startedNanos }

    fun clockUs(nowNanos: Long, read: () -> PlayedAudio?): Long? {
        if (latest == null || nowNanos - polledAtNanos >= POLL_INTERVAL_NANOS) poll(nowNanos, read)
        return latest?.let { TunnelClock.audioUs(it.frames, nowNanos - it.atNanos) }
    }

    fun rate(): Double? {
        val from = first ?: return null
        val to = latest ?: return null
        if (to.atNanos == from.atNanos) return null
        val playedSeconds = (to.frames - from.frames).toDouble() / TunnelClock.SAMPLE_RATE
        return playedSeconds / ((to.atNanos - from.atNanos).toDouble() / NANOS_PER_SECOND)
    }

    private fun poll(nowNanos: Long, read: () -> PlayedAudio?) {
        polledAtNanos = nowNanos
        val reading = read() ?: return
        if (reading.frames <= 0L) return
        if (first == null) first = reading
        latest = reading
    }

    companion object {
        const val POLL_INTERVAL_NANOS = 200_000_000L
        private const val NANOS_PER_SECOND = 1_000_000_000L
    }
}
