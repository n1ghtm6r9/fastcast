package dev.fastcast.client.connect

import dev.fastcast.client.discovery.DiscoveredServer
import java.util.concurrent.TimeUnit

class AutoConnectPolicy(
    private val nowMs: () -> Long = { TimeUnit.NANOSECONDS.toMillis(System.nanoTime()) },
) {

    var isEnabled = true
        private set

    private var firstReplyAtMs: Long? = null
    private var holdUntilMs = RELEASED

    fun disable() {
        isEnabled = false
    }

    fun holdFor(durationMs: Long) {
        holdUntilMs = nowMs() + durationMs
    }

    fun holdUntilReleased() {
        holdUntilMs = Long.MAX_VALUE
    }

    fun release() {
        holdUntilMs = RELEASED
    }

    fun choose(servers: List<DiscoveredServer>, rememberedName: String?): DiscoveredServer? {
        if (!isEnabled) return null
        if (servers.isEmpty()) {
            firstReplyAtMs = null
            return null
        }
        val now = nowMs()
        val firstReplyMs = firstReplyAtMs ?: now.also { firstReplyAtMs = it }
        if (now < holdUntilMs) return null
        return servers.firstOrNull { it.name == rememberedName }
            ?: servers.singleOrNull()?.takeIf { now - firstReplyMs >= SETTLE_MS }
    }

    private companion object {
        const val SETTLE_MS = 1_500L
        const val RELEASED = Long.MIN_VALUE
    }
}
