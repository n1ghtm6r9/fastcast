package dev.fastcast.client.debug

interface DebugClock {

    fun nowNanos(): Long

    fun after(delayMs: Long, task: () -> Unit)
}
