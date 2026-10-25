package dev.fastcast.client.update

import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class RecordingUpdateListener : UpdateListener {

    private val events = LinkedBlockingQueue<String>()

    override fun onNoUpdate() {
        events.add(NO_UPDATE)
    }

    override fun onUpdateFound(manifest: UpdateManifest) {
        events.add("found ${manifest.versionName}")
    }

    override fun onUpdateProgress(percent: Int) {
        events.add("progress $percent")
    }

    override fun onUpdateFailed(cause: Exception) {
        events.add("failed ${cause.message}")
    }

    fun nextEvent(): String =
        events.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for an update event")

    fun hasNoEventWithin(timeoutMs: Long): Boolean = events.poll(timeoutMs, TimeUnit.MILLISECONDS) == null

    companion object {
        const val NO_UPDATE = "none"

        private const val TIMEOUT_SECONDS = 5L
    }
}
