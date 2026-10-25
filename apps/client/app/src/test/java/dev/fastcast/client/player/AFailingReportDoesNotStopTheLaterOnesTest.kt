package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AFailingReportDoesNotStopTheLaterOnesTest {

    private val sink = RecordingStatsSink()
    private val calls = AtomicInteger()
    private var reporter: StatsReporter? = null

    @After
    fun tearDown() {
        reporter?.stop()
    }

    private fun startedWith(
        snapshot: () -> StatsSnapshot = { aStatsSnapshot() },
        display: () -> String = { "3840x2160@60" },
        send: (String) -> Boolean = sink::send,
    ) {
        reporter = StatsReporter(snapshot, display, send, intervalMs = 20L).also(StatsReporter::start)
    }

    @Test
    fun `a snapshot that fails once is retried at the next interval`() {
        startedWith(snapshot = { if (calls.getAndIncrement() == 0) error("no snapshot") else aStatsSnapshot() })

        assertEquals("stats", sink.next().getString("type"))
    }

    @Test
    fun `a display lookup that fails once is retried at the next interval`() {
        startedWith(display = { if (calls.getAndIncrement() == 0) error("no display") else "3840x2160@60" })

        assertEquals("3840x2160@60", sink.next().getString("display"))
    }

    @Test
    fun `a send that fails once is retried at the next interval`() {
        startedWith(send = { text -> if (calls.getAndIncrement() == 0) error("socket closed") else sink.send(text) })

        assertEquals("stats", sink.next().getString("type"))
    }

    @Test
    fun `a send that is refused is not an error`() {
        startedWith(
            send = { text ->
                sink.send(text)
                false
            },
        )

        assertEquals("stats", sink.next().getString("type"))
        assertEquals("stats", sink.next().getString("type"))
    }
}
