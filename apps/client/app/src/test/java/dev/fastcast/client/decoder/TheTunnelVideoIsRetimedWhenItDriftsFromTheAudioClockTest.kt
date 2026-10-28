package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheTunnelVideoIsRetimedWhenItDriftsFromTheAudioClockTest {

    private fun slackAfter(frames: Int, schedule: TunnelSchedule = TunnelSchedule(DELAY_US), clockShiftUs: (Int) -> Long): Long {
        schedule.presentationUs(0L, CLOCK_US)
        var slackUs = 0L
        for (index in 1..frames) {
            val frameUs = index * FRAME_US
            val clockUs = CLOCK_US + frameUs + clockShiftUs(index)
            slackUs = schedule.presentationUs(frameUs, clockUs) - clockUs
        }
        return slackUs
    }

    @Test
    fun `video in step with the clock keeps its delay`() {
        assertEquals(DELAY_US, slackAfter(10_000) { 0L })
    }

    @Test
    fun `video that keeps falling behind the clock is moved later`() {
        val slackUs = slackAfter(2_000) { 40_000L }

        assertTrue("$slackUs", slackUs in 30_000L..DELAY_US)
    }

    @Test
    fun `video that runs far ahead of the clock is brought back near its delay`() {
        val slackUs = slackAfter(3_000) { -300_000L }

        assertTrue("$slackUs", slackUs in DELAY_US..DELAY_US + 25_000L)
    }

    @Test
    fun `a schedule without delay leaves a drift under two milliseconds alone`() {
        val schedule = TunnelSchedule(0L)
        schedule.presentationUs(0L, CLOCK_US)

        val moved = (1..2_000).count { index ->
            val frameUs = index * FRAME_US
            val clockUs = CLOCK_US + frameUs + 1_500L
            schedule.presentationUs(frameUs, clockUs) != CLOCK_US + frameUs
        }

        assertEquals(0, moved)
    }

    @Test
    fun `a short hiccup leaves the timeline alone`() {
        val schedule = TunnelSchedule(DELAY_US)
        schedule.presentationUs(0L, CLOCK_US)

        val moved = (1..2_000).count { index ->
            val frameUs = index * FRAME_US
            val clockUs = CLOCK_US + frameUs + if (index in 500 until 620) 50_000L else 0L
            schedule.presentationUs(frameUs, clockUs) != CLOCK_US + DELAY_US + frameUs
        }

        assertEquals(0, moved)
    }

    @Test
    fun `a slow clock drift is followed without straying far from the delay`() {
        val slackUs = slackAfter(100_000) { index -> index * FRAME_US / 10_000L }

        assertTrue("$slackUs", slackUs in 20_000L..DELAY_US)
    }

    @Test
    fun `a drift within half the delay is left alone`() {
        assertEquals(DELAY_US - 20_000L, slackAfter(3_000) { 20_000L })
    }

    @Test
    fun `a short video delay is retimed after a smaller drift`() {
        val slackUs = slackAfter(3_000, TunnelSchedule(12_000L)) { 8_000L }

        assertTrue("$slackUs", slackUs in 9_000L..12_000L)
    }

    private companion object {
        const val CLOCK_US = TunnelClock.AUDIO_START_US + 2_000_000L
        const val FRAME_US = 8_333L
        const val DELAY_US = 50_000L
    }
}
