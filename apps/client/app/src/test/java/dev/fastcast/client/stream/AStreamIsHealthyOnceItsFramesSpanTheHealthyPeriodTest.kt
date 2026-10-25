package dev.fastcast.client.stream

import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class AStreamIsHealthyOnceItsFramesSpanTheHealthyPeriodTest {

    private var nowMs = 0L
    private val health = StreamHealth(HEALTHY_AFTER_MS) { nowMs }

    @Test
    fun `a single frame proves nothing`() {
        assertFalse(health.frameDelivered())
    }

    @Test
    fun `frames spanning less than the period do not prove it`() {
        health.frameDelivered()

        nowMs = HEALTHY_AFTER_MS - 1

        assertFalse(health.frameDelivered())
    }

    @Test
    fun `the frame that completes the span proves it`() {
        health.frameDelivered()

        nowMs = HEALTHY_AFTER_MS

        assertTrue(health.frameDelivered())
    }

    @Test
    fun `a proven stream is reported only once`() {
        health.frameDelivered()
        nowMs = HEALTHY_AFTER_MS
        health.frameDelivered()

        nowMs = 2 * HEALTHY_AFTER_MS

        assertFalse(health.frameDelivered())
    }

    @Test
    fun `a restart forgets the frames of the previous connection`() {
        health.frameDelivered()
        nowMs = HEALTHY_AFTER_MS - 1
        health.restart()

        nowMs = HEALTHY_AFTER_MS

        assertFalse(health.frameDelivered())
    }

    @Test
    fun `a restarted connection can prove itself again`() {
        health.frameDelivered()
        nowMs = HEALTHY_AFTER_MS
        health.frameDelivered()
        health.restart()
        health.frameDelivered()

        nowMs = 2 * HEALTHY_AFTER_MS

        assertTrue(health.frameDelivered())
    }

    private companion object {
        const val HEALTHY_AFTER_MS = 5_000L
    }
}
