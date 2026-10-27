package dev.fastcast.client.decoder

enum class DecoderTweak {
    NONE,
    LOW_LATENCY,
    DISCARD_OUTPUT,
    STRETCHED_TIMESTAMPS,
    ;

    val lowLatency: Boolean
        get() = this == LOW_LATENCY

    val rendersOutput: Boolean
        get() = this != DISCARD_OUTPUT

    fun presentationTimeUs(ptsUs: Long): Long = if (this == STRETCHED_TIMESTAMPS) ptsUs * STRETCH else ptsUs

    private companion object {
        const val STRETCH = 2L
    }
}
