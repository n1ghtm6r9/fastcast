package dev.fastcast.client.discovery

import java.util.concurrent.Executor
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class QueueingExecutor : Executor {

    private val queued = LinkedBlockingQueue<Runnable>()

    override fun execute(command: Runnable) {
        queued.add(command)
    }

    fun next(): Runnable =
        queued.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for a queued task")

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
