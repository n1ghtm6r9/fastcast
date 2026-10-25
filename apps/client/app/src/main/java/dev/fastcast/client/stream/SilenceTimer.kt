package dev.fastcast.client.stream

import java.util.concurrent.TimeUnit

class SilenceTimer(
    private val limitMs: Long,
    private val nowMs: () -> Long = { TimeUnit.NANOSECONDS.toMillis(System.nanoTime()) },
) {

    @Volatile
    private var lastActivityMs = nowMs()

    fun touch() {
        lastActivityMs = nowMs()
    }

    fun hasExpired(): Boolean = nowMs() - lastActivityMs >= limitMs
}
