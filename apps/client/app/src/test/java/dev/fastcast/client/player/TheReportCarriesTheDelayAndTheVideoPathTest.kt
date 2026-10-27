package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheReportCarriesTheDelayAndTheVideoPathTest {

    private val sink = RecordingStatsSink()
    private val reporter = StatsReporter(
        snapshot = { aStatsSnapshot(delayMs = 41.26, tunneled = true) },
        display = { "3840x2160@60" },
        send = sink::send,
        intervalMs = 60_000L,
    )

    @After
    fun tearDown() {
        reporter.stop()
    }

    @Test
    fun `a report carries how long the tv held a frame and whether it used its video path`() {
        reporter.reportNow()

        val json = sink.next()

        assertEquals(41.3, json.getDouble("delayMs"), 0.0)
        assertTrue(json.getBoolean("tunneled"))
    }
}
