package dev.fastcast.client.stream

import java.util.concurrent.TimeUnit

class StreamHealth(
    private val healthyAfterMs: Long = DEFAULT_HEALTHY_AFTER_MS,
    private val nowMs: () -> Long = { TimeUnit.NANOSECONDS.toMillis(System.nanoTime()) },
) {

    private var firstFrameMs: Long? = null
    private var proven = false

    @Synchronized
    fun restart() {
        firstFrameMs = null
        proven = false
    }

    @Synchronized
    fun frameDelivered(): Boolean {
        val now = nowMs()
        val first = firstFrameMs ?: now.also { firstFrameMs = it }
        if (proven || now - first < healthyAfterMs) return false
        proven = true
        return true
    }

    companion object {
        const val DEFAULT_HEALTHY_AFTER_MS = 5_000L
    }
}
