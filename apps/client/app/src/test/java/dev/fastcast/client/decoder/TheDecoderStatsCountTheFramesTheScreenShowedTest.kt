package dev.fastcast.client.decoder

import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class TheDecoderStatsCountTheFramesTheScreenShowedTest {

    @Test
    fun `the frames shown are the ones the screen reported`() {
        val stats = DecoderStats()
        stats.framesRendered.addAndGet(5L)
        stats.framePresented(1L, 10L)
        stats.framePresented(2L, 20L)

        assertEquals(2L, stats.framesShown)
    }

    @Test
    fun `a screen that reports nothing is taken to show every frame handed to it`() {
        val stats = DecoderStats()
        stats.framesRendered.addAndGet(5L)

        assertEquals(5L, stats.framesShown)
    }

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
}
