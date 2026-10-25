package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ThePeriodicReportRepeatsUntilTheReporterIsStoppedTest {

    private val sink = RecordingStatsSink()
    private val reporters = mutableListOf<StatsReporter>()

    @After
    fun tearDown() {
        reporters.forEach(StatsReporter::stop)
    }

    private fun startedEvery(intervalMs: Long): StatsReporter {
        val reporter = StatsReporter({ aStatsSnapshot() }, { "3840x2160@60" }, sink::send, intervalMs)
        reporters.add(reporter)
        reporter.start()
        return reporter
    }

    @Test
    fun `nothing is sent before the first interval has passed`() {
        startedEvery(60_000L)

        assertTrue(sink.hasNothingWithin(200L))
    }

    @Test
    fun `a report is sent after every interval`() {
        startedEvery(30L)

        val reports = List(3) { sink.next() }

        assertEquals(listOf("stats", "stats", "stats"), reports.map { it.getString("type") })
        assertTrue(reports.none { it.has("error") })
    }

    @Test
    fun `a stopped reporter sends nothing more`() {
        val reporter = startedEvery(20L)
        sink.next()

        reporter.stop()
        Thread.sleep(100L)
        sink.discardPending()

        assertTrue(sink.hasNothingWithin(200L))
    }

    @Test
    fun `stopping twice is harmless`() {
        val reporter = startedEvery(60_000L)

        reporter.stop()
        reporter.stop()

        assertTrue(sink.hasNothingWithin(50L))
    }

    @Test
    fun `a report asked for after the stop is not sent`() {
        val reporter = startedEvery(60_000L)
        reporter.stop()

        reporter.reportNow()

        assertTrue(sink.hasNothingWithin(200L))
    }

    @Test
    fun `a failure reported after the stop is not sent and runs no follow up`() {
        val reporter = startedEvery(60_000L)
        reporter.stop()
        var followedUp = false

        reporter.reportFailure("late") { followedUp = true }

        assertTrue(sink.hasNothingWithin(200L))
        assertFalse(followedUp)
    }
}
