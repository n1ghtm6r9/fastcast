package dev.fastcast.client.testing

import java.util.concurrent.TimeUnit

private const val POLL_INTERVAL_MS = 5L

fun awaitTrue(timeoutMs: Long = 5_000L, condition: () -> Boolean): Boolean {
    val deadlineNs = System.nanoTime() + TimeUnit.MILLISECONDS.toNanos(timeoutMs)
    while (System.nanoTime() < deadlineNs) {
        if (condition()) return true
        Thread.sleep(POLL_INTERVAL_MS)
    }
    return condition()
}
