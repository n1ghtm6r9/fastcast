package dev.fastcast.client.decoder

object TunnelClock {

    private const val CHANNELS = 2
    private const val BYTES_PER_SAMPLE = 2
    private const val MILLISECONDS_PER_SECOND = 1_000L
    private const val MICROS_PER_SECOND = 1_000_000L
    private const val NANOS_PER_MICROSECOND = 1_000L
    private const val NANOS_PER_MILLISECOND = 1_000_000L

    const val AUDIO_START_US = 1_000_000_000_000L
    const val VIDEO_DELAY_US = 50_000L
    const val SAMPLE_RATE = 48_000
    const val CHUNK_MS = 10L
    const val CHUNK_BYTES = (SAMPLE_RATE * CHUNK_MS / MILLISECONDS_PER_SECOND).toInt() * CHANNELS * BYTES_PER_SAMPLE

    fun chunkNanos(index: Long): Long = AUDIO_START_US * NANOS_PER_MICROSECOND + index * CHUNK_MS * NANOS_PER_MILLISECOND

    fun audioUs(framesPlayed: Long, sinceNanos: Long): Long =
        AUDIO_START_US + framesPlayed * MICROS_PER_SECOND / SAMPLE_RATE + sinceNanos / NANOS_PER_MICROSECOND
}
