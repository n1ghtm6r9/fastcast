package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheRenderDelaysTellHowLongAFrameWaitedForTheScreenTest {

    @Test
    fun `the delay is the middle of the queue to screen times`() {
        val delays = RenderDelays(capacity = 8)
        delays.queued(1_000L, 0L)
        delays.queued(2_000L, 8_000_000L)
        delays.queued(3_000L, 16_000_000L)
        delays.rendered(1_000L, 20_000_000L)
        delays.rendered(2_000L, 38_000_000L)
        delays.rendered(3_000L, 41_000_000L)

        assertEquals(25.0, requireNotNull(delays.medianMs()), 1e-9)
    }

    @Test
    fun `an even count takes the mean of the two middle delays`() {
        val delays = RenderDelays(capacity = 8)
        listOf(10L, 40L, 20L, 30L).forEachIndexed { index, delayMs ->
            delays.queued(index.toLong(), 0L)
            delays.rendered(index.toLong(), delayMs * 1_000_000L)
        }

        assertEquals(25.0, requireNotNull(delays.medianMs()), 1e-9)
    }

    @Test
    fun `a shown frame tells how long it waited`() {
        val delays = RenderDelays(capacity = 8)
        delays.queued(1_000L, 5_000_000L)

        assertEquals(20_000_000L, delays.rendered(1_000L, 25_000_000L))
    }

    @Test
    fun `a frame shown without being queued tells nothing`() {
        assertNull(RenderDelays(capacity = 8).rendered(1_000L, 25_000_000L))
    }

    @Test
    fun `a frame shown without being queued is not counted`() {
        val delays = RenderDelays(capacity = 8)
        delays.rendered(1_000L, 5_000_000L)

        assertNull(delays.medianMs())
    }

    @Test
    fun `nothing shown means no delay`() {
        val delays = RenderDelays(capacity = 8)
        delays.queued(1_000L, 0L)

        assertNull(delays.medianMs())
    }

    @Test
    fun `only the newest delays count`() {
        val delays = RenderDelays(capacity = 2)
        listOf(100L, 10L, 20L).forEachIndexed { index, delayMs ->
            delays.queued(index.toLong(), 0L)
            delays.rendered(index.toLong(), delayMs * 1_000_000L)
        }

        assertEquals(15.0, requireNotNull(delays.medianMs()), 1e-9)
    }

    @Test
    fun `a frame queued long ago is forgotten`() {
        val delays = RenderDelays(capacity = 2)
        (1L..3L).forEach { delays.queued(it, 0L) }
        delays.rendered(1L, 50_000_000L)

        assertNull(delays.medianMs())
    }

    @Test
    fun `a frame is counted once even if it is reported twice`() {
        val delays = RenderDelays(capacity = 8)
        delays.queued(1L, 0L)
        delays.queued(2L, 0L)
        delays.rendered(1L, 10_000_000L)
        delays.rendered(1L, 90_000_000L)
        delays.rendered(2L, 20_000_000L)

        assertEquals(15.0, requireNotNull(delays.medianMs()), 1e-9)
    }
}
