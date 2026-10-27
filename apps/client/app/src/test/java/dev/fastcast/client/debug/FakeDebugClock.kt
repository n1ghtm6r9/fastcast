package dev.fastcast.client.debug

class FakeDebugClock : DebugClock {

    var nowMs = 0L
        private set

    private val tasks = mutableListOf<Pair<Long, () -> Unit>>()

    override fun nowNanos(): Long = nowMs * NANOS_PER_MILLISECOND

    override fun after(delayMs: Long, task: () -> Unit) {
        tasks += nowMs + delayMs to task
    }

    fun advanceBy(delayMs: Long) {
        val endMs = nowMs + delayMs
        while (true) {
            val next = tasks.filter { it.first <= endMs }.minByOrNull { it.first } ?: break
            tasks.remove(next)
            nowMs = next.first
            next.second()
        }
        nowMs = endMs
    }

    companion object {
        private const val NANOS_PER_MILLISECOND = 1_000_000L
    }
}
