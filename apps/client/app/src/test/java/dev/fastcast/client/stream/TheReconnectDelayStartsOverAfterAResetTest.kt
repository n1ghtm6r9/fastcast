package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Test

class TheReconnectDelayStartsOverAfterAResetTest {

    private val backoff = ReconnectBackoff()

    @Test
    fun `the first delay is used again after a reset`() {
        repeat(5) { backoff.next() }

        backoff.reset()

        assertEquals(1_000L, backoff.next())
        assertEquals(2_000L, backoff.next())
    }

    @Test
    fun `a reset before any attempt changes nothing`() {
        backoff.reset()

        assertEquals(1_000L, backoff.next())
    }
}
