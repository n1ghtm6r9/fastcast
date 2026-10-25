package dev.fastcast.client.discovery

import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class RecordingServerFinderListener : ServerFinderListener {

    private val reports = LinkedBlockingQueue<List<DiscoveredServer>>()

    override fun onServersChanged(servers: List<DiscoveredServer>) {
        reports.add(servers)
    }

    fun nextReport(): List<DiscoveredServer> =
        reports.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for a report")

    fun hasNoReportWithin(timeoutMs: Long): Boolean = reports.poll(timeoutMs, TimeUnit.MILLISECONDS) == null

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
