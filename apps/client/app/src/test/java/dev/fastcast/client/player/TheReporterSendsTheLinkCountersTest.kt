package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import java.util.concurrent.atomic.AtomicReference
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test

class TheReporterSendsTheLinkCountersTest {

    private val sink = RecordingStatsSink()
    private val socketDrops = AtomicReference<Int?>(417)
    private val inboxDrops = AtomicReference<Int?>(23)
    private val linkPauseMs = AtomicReference<Int?>(140)
    private val reporter = StatsReporter(
        snapshot = {
            aStatsSnapshot(
                socketDrops = socketDrops.get(),
                inboxDrops = inboxDrops.get(),
                linkPauseMs = linkPauseMs.get(),
            )
        },
        display = { "2560x1440@120" },
        send = sink::send,
        intervalMs = 60_000L,
    )

    @After
    fun tearDown() {
        reporter.stop()
    }

    @Test
    fun `a report carries the socket drops of the link`() {
        reporter.reportNow()

        assertEquals(417, sink.next().getInt("socketDrops"))
    }

    @Test
    fun `a report carries the inbox drops of the link`() {
        reporter.reportNow()

        assertEquals(23, sink.next().getInt("inboxDrops"))
    }

    @Test
    fun `a report carries the longest pause of the link`() {
        reporter.reportNow()

        assertEquals(140, sink.next().getInt("linkPauseMs"))
    }

    @Test
    fun `a report leaves out the counters the link does not know`() {
        socketDrops.set(null)
        inboxDrops.set(null)
        linkPauseMs.set(null)

        reporter.reportNow()

        val json = sink.next()
        assertFalse(json.has("socketDrops"))
        assertFalse(json.has("inboxDrops"))
        assertFalse(json.has("linkPauseMs"))
    }
}
