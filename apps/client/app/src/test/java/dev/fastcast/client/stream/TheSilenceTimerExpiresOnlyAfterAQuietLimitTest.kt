package dev.fastcast.client.stream

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheSilenceTimerExpiresOnlyAfterAQuietLimitTest {

    private var nowMs = 0L
    private val timer = SilenceTimer(LIMIT_MS, LIMIT_MS) { nowMs }

    @Test
    fun `a timer just short of the limit has not expired`() {
        nowMs = LIMIT_MS - 1

        assertFalse(timer.hasExpired())
    }

    @Test
    fun `a timer expires once the limit passes without activity`() {
        nowMs = LIMIT_MS

        assertTrue(timer.hasExpired())
    }

    @Test
    fun `activity starts the limit over`() {
        nowMs = LIMIT_MS - 1
        timer.touch()

        nowMs = 2 * LIMIT_MS - 2

        assertFalse(timer.hasExpired())
    }

    private companion object {
        const val LIMIT_MS = 5_000L
    }
}
