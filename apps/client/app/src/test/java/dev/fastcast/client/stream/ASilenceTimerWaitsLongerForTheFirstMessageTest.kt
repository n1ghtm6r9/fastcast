package dev.fastcast.client.stream

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ASilenceTimerWaitsLongerForTheFirstMessageTest {

    private var nowMs = 0L
    private val timer = SilenceTimer(FIRST_MESSAGE_LIMIT_MS, STREAMING_LIMIT_MS) { nowMs }

    @Test
    fun `before the first message the streaming limit does not expire the timer`() {
        nowMs = STREAMING_LIMIT_MS

        assertFalse(timer.hasExpired())
    }

    @Test
    fun `just short of the first message limit the timer has not expired`() {
        nowMs = FIRST_MESSAGE_LIMIT_MS - 1

        assertFalse(timer.hasExpired())
    }

    @Test
    fun `without a first message the timer expires at the longer limit`() {
        nowMs = FIRST_MESSAGE_LIMIT_MS

        assertTrue(timer.hasExpired())
    }

    @Test
    fun `once a message arrived the streaming limit applies`() {
        nowMs = 2_000L
        timer.touch()

        nowMs = 2_000L + STREAMING_LIMIT_MS - 1
        val justShort = timer.hasExpired()
        nowMs = 2_000L + STREAMING_LIMIT_MS
        val atTheLimit = timer.hasExpired()

        assertFalse(justShort)
        assertTrue(atTheLimit)
    }

    @Test
    fun `a restart waits for the first message again`() {
        timer.touch()
        nowMs = 2_000L
        timer.restart()

        nowMs = 2_000L + STREAMING_LIMIT_MS

        assertFalse(timer.hasExpired())
    }

    @Test
    fun `a restart counts the longer limit from the restart`() {
        timer.touch()
        nowMs = 2_000L
        timer.restart()

        nowMs = 2_000L + FIRST_MESSAGE_LIMIT_MS

        assertTrue(timer.hasExpired())
    }

    private companion object {
        const val FIRST_MESSAGE_LIMIT_MS = 15_000L
        const val STREAMING_LIMIT_MS = 5_000L
    }
}
