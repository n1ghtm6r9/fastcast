package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheFrameLatenessTellsHowLateFramesWereShownTest {

    @Test
    fun `the median is the middle lateness`() {
        val lateness = FrameLateness(capacity = 8)
        listOf(9_000L, 1_000L, 3_000L).forEach(lateness::add)

        assertEquals(3.0, requireNotNull(lateness.percentileMs(0.5)), 1e-9)
    }

    @Test
    fun `a high share reaches into the latest frames`() {
        val lateness = FrameLateness(capacity = 16)
        (1L..10L).forEach { lateness.add(it * 1_000L) }

        assertEquals(9.0, requireNotNull(lateness.percentileMs(0.9)), 1e-9)
    }

    @Test
    fun `frames shown before their time count as early`() {
        val lateness = FrameLateness(capacity = 8)
        lateness.add(-2_500L)

        assertEquals(-2.5, requireNotNull(lateness.percentileMs(0.5)), 1e-9)
    }

    @Test
    fun `only the newest frames are kept`() {
        val lateness = FrameLateness(capacity = 2)
        listOf(100_000L, 1_000L, 3_000L).forEach(lateness::add)

        assertEquals(3.0, requireNotNull(lateness.percentileMs(1.0)), 1e-9)
    }

    @Test
    fun `nothing shown means no lateness`() {
        assertNull(FrameLateness(capacity = 8).percentileMs(0.5))
    }
}
