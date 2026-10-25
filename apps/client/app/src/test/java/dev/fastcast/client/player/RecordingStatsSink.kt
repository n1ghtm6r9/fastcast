package dev.fastcast.client.player

import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import org.json.JSONObject

class RecordingStatsSink {

    private val messages = LinkedBlockingQueue<String>()

    fun send(text: String): Boolean = messages.add(text)

    fun next(): JSONObject =
        JSONObject(messages.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for a stats message"))

    fun hasNothingWithin(timeoutMs: Long): Boolean = messages.poll(timeoutMs, TimeUnit.MILLISECONDS) == null

    fun discardPending() = messages.clear()

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
