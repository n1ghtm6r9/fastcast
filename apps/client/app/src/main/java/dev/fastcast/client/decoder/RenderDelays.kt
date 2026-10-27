package dev.fastcast.client.decoder

class RenderDelays(private val capacity: Int) {

    private val queuedAt = LinkedHashMap<Long, Long>()
    private val delays = ArrayDeque<Long>()

    @Synchronized
    fun queued(ptsUs: Long, nanos: Long) {
        queuedAt[ptsUs] = nanos
        if (queuedAt.size > capacity) queuedAt.remove(queuedAt.keys.first())
    }

    @Synchronized
    fun rendered(ptsUs: Long, nanos: Long) {
        val queuedNanos = queuedAt.remove(ptsUs) ?: return
        delays.addLast(nanos - queuedNanos)
        if (delays.size > capacity) delays.removeFirst()
    }

    @Synchronized
    fun medianMs(): Double? {
        if (delays.isEmpty()) return null
        val sorted = delays.sorted()
        val middle = sorted.size / 2
        val nanos = if (sorted.size % 2 == 0) (sorted[middle - 1] + sorted[middle]) / 2.0 else sorted[middle].toDouble()
        return nanos / NANOS_PER_MILLISECOND
    }

    private companion object {
        const val NANOS_PER_MILLISECOND = 1_000_000.0
    }
}
