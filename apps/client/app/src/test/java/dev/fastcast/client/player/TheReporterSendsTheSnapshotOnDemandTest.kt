package dev.fastcast.client.player

import dev.fastcast.client.testing.aStatsSnapshot
import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.atomic.AtomicLong
import java.util.concurrent.atomic.AtomicReference
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheReporterSendsTheSnapshotOnDemandTest {

    private val sink = RecordingStatsSink()
    private val dropped = AtomicLong(0L)
    private val receiveBuffer = AtomicInteger(8_388_608)
    private val display = AtomicReference("3840x2160@60")
    private val network = AtomicReference<String?>("ethernet 100 Mbit/s")
    private val reporter = StatsReporter(
        snapshot = { aStatsSnapshot(droppedFrames = dropped.get(), receiveBufferBytes = receiveBuffer.get()) },
        display = display::get,
        send = sink::send,
        intervalMs = 60_000L,
        network = network::get,
    )

    @After
    fun tearDown() {
        reporter.stop()
    }

    @Test
    fun `a report carries the snapshot and the display mode`() {
        reporter.reportNow()

        val json = sink.next()

        assertEquals("stats", json.getString("type"))
        assertEquals("c2.mtk.hevc.decoder", json.getString("decoder"))
        assertEquals("video/hevc", json.getString("mime"))
        assertEquals(3840, json.getInt("width"))
        assertEquals(2160, json.getInt("height"))
        assertEquals(59.9, json.getDouble("fps"), 0.0)
        assertEquals(31.2, json.getDouble("mbps"), 0.0)
        assertEquals(0L, json.getLong("dropped"))
        assertEquals("3840x2160@60", json.getString("display"))
    }

    @Test
    fun `a report carries the colors of the decoder output`() {
        reporter.reportNow()

        val json = sink.next()

        assertEquals(1, json.getInt("colorStandard"))
        assertEquals(2, json.getInt("colorRange"))
        assertEquals(3, json.getInt("colorTransfer"))
    }

    @Test
    fun `a report carries the receive buffer of the link`() {
        reporter.reportNow()

        assertEquals(8_388_608, sink.next().getInt("receiveBuffer"))
    }

    @Test
    fun `a report leaves out a receive buffer that is not known`() {
        receiveBuffer.set(0)

        reporter.reportNow()

        assertFalse(sink.next().has("receiveBuffer"))
    }

    @Test
    fun `a report carries the network of the tv`() {
        reporter.reportNow()

        assertEquals("ethernet 100 Mbit/s", sink.next().getString("network"))
    }

    @Test
    fun `a report leaves out a network that is not known`() {
        network.set(null)

        reporter.reportNow()

        assertFalse(sink.next().has("network"))
    }

    @Test
    fun `a plain report has no error`() {
        reporter.reportNow()

        assertFalse(sink.next().has("error"))
    }

    @Test
    fun `a report is sent once`() {
        reporter.reportNow()
        sink.next()

        assertTrue(sink.hasNothingWithin(200L))
    }

    @Test
    fun `every report reads the snapshot and the display mode afresh`() {
        reporter.reportNow()
        val first = sink.next()
        dropped.set(7L)
        display.set("1920x1080@30")

        reporter.reportNow()
        val second = sink.next()

        assertEquals(0L, first.getLong("dropped"))
        assertEquals("3840x2160@60", first.getString("display"))
        assertEquals(7L, second.getLong("dropped"))
        assertEquals("1920x1080@30", second.getString("display"))
    }
}
