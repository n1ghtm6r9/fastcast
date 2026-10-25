package dev.fastcast.client.stream

import java.util.concurrent.TimeUnit

class SilenceTimer(
    private val firstMessageLimitMs: Long = DEFAULT_FIRST_MESSAGE_LIMIT_MS,
    private val streamingLimitMs: Long = DEFAULT_STREAMING_LIMIT_MS,
    private val nowMs: () -> Long = { TimeUnit.NANOSECONDS.toMillis(System.nanoTime()) },
) {

    val checkPeriodMs: Long = (minOf(firstMessageLimitMs, streamingLimitMs) / CHECKS_PER_LIMIT).coerceAtLeast(1L)

    private var lastActivityMs = nowMs()
    private var streaming = false

    @Synchronized
    fun restart() {
        lastActivityMs = nowMs()
        streaming = false
    }

    @Synchronized
    fun touch() {
        lastActivityMs = nowMs()
        streaming = true
    }

    @Synchronized
    fun hasExpired(): Boolean {
        val limitMs = if (streaming) streamingLimitMs else firstMessageLimitMs
        return nowMs() - lastActivityMs >= limitMs
    }

    private companion object {
        const val DEFAULT_FIRST_MESSAGE_LIMIT_MS = 15_000L
        const val DEFAULT_STREAMING_LIMIT_MS = 5_000L
        const val CHECKS_PER_LIMIT = 50
    }
}
