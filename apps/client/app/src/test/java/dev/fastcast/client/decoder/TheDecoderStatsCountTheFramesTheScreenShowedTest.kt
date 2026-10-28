package dev.fastcast.client.decoder

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class TheDecoderStatsCountTheFramesTheScreenShowedTest {

    @Test
    fun `a queued frame is counted`() {
        val stats = DecoderStats()
        stats.frameQueued(1L, 0L)
        stats.frameQueued(2L, 0L)

        assertEquals(2L, stats.framesQueued.get())
    }

    @Test
    fun `a presented frame keeps its screen time`() {
        val stats = DecoderStats()
        stats.framePresented(1L, 30_000_000L)

        assertArrayEquals(longArrayOf(30_000_000L), stats.presented.since(0L))
    }

    @Test
    fun `a presented frame is timed from when it was queued`() {
        val stats = DecoderStats()
        stats.frameQueued(7L, 1_000_000L)
        stats.framePresented(7L, 31_000_000L)

        assertEquals(30.0, requireNotNull(stats.delays.medianMs()), 1e-9)
    }

    @Test
    fun `a decoded frame is counted and timed from when it was queued`() {
        val stats = DecoderStats()
        stats.frameQueued(7L, 1_000_000L)
        stats.frameDecoded(7L, 5_000_000L)

        assertEquals(1L, stats.framesDecoded.get())
        assertEquals(4.0, requireNotNull(stats.decodeDelays.medianMs()), 1e-9)
    }
}
