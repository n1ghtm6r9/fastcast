package dev.fastcast.client.stream

class ReconnectBackoff(private val delaysMs: List<Long> = DEFAULT_DELAYS_MS) {

    private var attempt = 0

    @Synchronized
    fun next(): Long {
        val delay = delaysMs[attempt]
        attempt = minOf(attempt + 1, delaysMs.lastIndex)
        return delay
    }

    @Synchronized
    fun reset() {
        attempt = 0
    }

    companion object {
        val DEFAULT_DELAYS_MS = listOf(1_000L, 2_000L, 5_000L)
    }
}
