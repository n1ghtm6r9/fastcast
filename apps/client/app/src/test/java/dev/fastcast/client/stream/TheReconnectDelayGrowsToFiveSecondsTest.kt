package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Test

class TheReconnectDelayGrowsToFiveSecondsTest {

    private val backoff = ReconnectBackoff()

    @Test
    fun `the delays are one then two then five seconds`() {
        assertEquals(1_000L, backoff.next())
        assertEquals(2_000L, backoff.next())
        assertEquals(5_000L, backoff.next())
    }

    @Test
    fun `the delay never grows beyond five seconds`() {
        repeat(3) { backoff.next() }

        repeat(10) { assertEquals(5_000L, backoff.next()) }
    }
}
