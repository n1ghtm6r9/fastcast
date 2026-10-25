package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import dev.fastcast.client.testing.awaitTrue
import java.util.concurrent.CopyOnWriteArrayList
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicLong
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AFailureIsReportedBeforeTheFollowUpRunsTest {

    private val sink = RecordingStatsSink()
    private val events = CopyOnWriteArrayList<String>()
    private var reporter: StatsReporter? = null

    @After
    fun tearDown() {
        reporter?.stop()
    }

    private fun reporterWith(
        flushDelayMs: Long = 20L,
        snapshot: () -> StatsSnapshot = { aStatsSnapshot() },
    ): StatsReporter = StatsReporter(
        snapshot = snapshot,
        display = { "3840x2160@60" },
        send = { text ->
            events.add(REPORT)
            sink.send(text)
        },
        intervalMs = 60_000L,
        flushDelayMs = flushDelayMs,
    ).also { reporter = it }

    @Test
    fun `the report carries the error text`() {
        reporterWith().reportFailure("Decoder failed") {}

        assertEquals("Decoder failed", sink.next().getString("error"))
    }

    @Test
    fun `the report still carries the usual fields`() {
        reporterWith().reportFailure("Decoder failed") {}

        val json = sink.next()

        assertEquals("stats", json.getString("type"))
        assertEquals("c2.mtk.hevc.decoder", json.getString("decoder"))
        assertEquals("3840x2160@60", json.getString("display"))
    }

    @Test
    fun `the follow up runs after the report was sent`() {
        reporterWith().reportFailure("Decoder failed") { events.add(FOLLOW_UP) }

        assertTrue(awaitTrue { events.size == 2 })
        assertEquals(listOf(REPORT, FOLLOW_UP), events.toList())
    }

    @Test
    fun `the follow up waits for the flush delay`() {
        val finishedAt = AtomicLong()
        val finished = CountDownLatch(1)
        val startedAt = System.nanoTime()

        reporterWith(flushDelayMs = 300L).reportFailure("Decoder failed") {
            finishedAt.set(System.nanoTime())
            finished.countDown()
        }

        assertTrue(finished.await(5, TimeUnit.SECONDS))
        assertTrue(TimeUnit.NANOSECONDS.toMillis(finishedAt.get() - startedAt) >= 300L)
    }

    @Test
    fun `the follow up runs even when the report cannot be built`() {
        val reporter = reporterWith(snapshot = { error("no snapshot") })

        reporter.reportFailure("Decoder failed") { events.add(FOLLOW_UP) }

        assertTrue(awaitTrue { events.contains(FOLLOW_UP) })
        assertTrue(sink.hasNothingWithin(100L))
    }

    private companion object {
        const val REPORT = "report"
        const val FOLLOW_UP = "follow-up"
    }
}
