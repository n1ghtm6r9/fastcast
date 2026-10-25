package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheSilenceTimerIsCheckedFarMoreOftenThanItsShortestLimitTest {

    @Test
    fun `the production timer is checked ten times a second`() {
        assertEquals(100L, SilenceTimer().checkPeriodMs)
    }

    @Test
    fun `the period follows the shortest limit and not the longer one`() {
        assertEquals(6L, SilenceTimer(1_500L, 300L).checkPeriodMs)
        assertEquals(6L, SilenceTimer(300L, 1_500L).checkPeriodMs)
    }

    @Test
    fun `a tiny limit still gives a period the scheduler accepts`() {
        assertTrue(SilenceTimer(10L, 10L).checkPeriodMs >= 1L)
    }
}
