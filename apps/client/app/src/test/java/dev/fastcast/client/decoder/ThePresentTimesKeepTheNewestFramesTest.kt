package dev.fastcast.client.decoder

import org.junit.Assert.assertArrayEquals
import org.junit.Test

class ThePresentTimesKeepTheNewestFramesTest {

    @Test
    fun `the times since the start come back in order`() {
        val times = PresentTimes(capacity = 8)
        listOf(10L, 20L, 30L, 40L).forEach(times::add)

        assertArrayEquals(longArrayOf(30L, 40L), times.since(25L))
    }

    @Test
    fun `a time right at the start counts`() {
        val times = PresentTimes(capacity = 8)
        times.add(25L)

        assertArrayEquals(longArrayOf(25L), times.since(25L))
    }

    @Test
    fun `only the newest times are kept once the buffer is full`() {
        val times = PresentTimes(capacity = 3)
        (1L..5L).forEach(times::add)

        assertArrayEquals(longArrayOf(3L, 4L, 5L), times.since(0L))
    }

    @Test
    fun `nothing recorded means no times`() {
        assertArrayEquals(LongArray(0), PresentTimes(capacity = 3).since(0L))
    }
}
