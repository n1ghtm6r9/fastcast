package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelVideoIsAnchoredToTheRunningAudioClockTest {

    private val schedule = TunnelSchedule()

    @Test
    fun `the first frame is due a video delay after the running audio clock`() {
        assertEquals(CLOCK_US + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(8_000L, CLOCK_US))
    }

    @Test
    fun `later frames keep their spacing`() {
        schedule.presentationUs(8_000L, CLOCK_US)

        val presentationUs = schedule.presentationUs(16_333L, CLOCK_US + 8_333L)

        assertEquals(CLOCK_US + TunnelClock.VIDEO_DELAY_US + 8_333L, presentationUs)
    }

    @Test
    fun `before the clock runs the first frame is due a video delay after the audio starts`() {
        assertEquals(TunnelClock.AUDIO_START_US + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(8_000L, null))
    }

    @Test
    fun `the video moves onto the clock once it runs`() {
        schedule.presentationUs(0L, null)
        val clockUs = TunnelClock.AUDIO_START_US + 300_000L

        assertEquals(clockUs + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(8_333L, clockUs))
    }

    @Test
    fun `a frame without a clock reading keeps the timeline`() {
        schedule.presentationUs(0L, CLOCK_US)

        assertEquals(CLOCK_US + TunnelClock.VIDEO_DELAY_US + 8_333L, schedule.presentationUs(8_333L, null))
    }

    @Test
    fun `a shorter video delay brings the frames closer to the clock`() {
        assertEquals(CLOCK_US + 12_000L, TunnelSchedule(12_000L).presentationUs(8_000L, CLOCK_US))
    }

    @Test
    fun `a video delay below zero times the frames behind the running clock`() {
        assertEquals(CLOCK_US - 16_000L, TunnelSchedule(-16_000L).presentationUs(8_000L, CLOCK_US))
    }

    @Test
    fun `before the clock runs a frame is never due before the audio starts`() {
        assertEquals(TunnelClock.AUDIO_START_US, TunnelSchedule(-16_000L).presentationUs(8_000L, null))
    }

    @Test
    fun `a stream that starts its time again is anchored afresh`() {
        schedule.presentationUs(60_000_000L, CLOCK_US)
        val clockUs = CLOCK_US + 8_333L

        assertEquals(clockUs + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(0L, clockUs))
    }

    @Test
    fun `a stream whose time leaps ahead is anchored afresh`() {
        schedule.presentationUs(0L, CLOCK_US)
        val clockUs = CLOCK_US + 8_333L

        assertEquals(clockUs + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(60_000_000L, clockUs))
    }

    @Test
    fun `frames lost for a moment keep the timeline`() {
        schedule.presentationUs(0L, CLOCK_US)

        val presentationUs = schedule.presentationUs(500_000L, CLOCK_US + 503_000L)

        assertEquals(CLOCK_US + TunnelClock.VIDEO_DELAY_US + 500_000L, presentationUs)
    }

    @Test
    fun `a restart anchors the next frame afresh`() {
        schedule.presentationUs(0L, CLOCK_US)
        schedule.restart()
        val clockUs = TunnelClock.AUDIO_START_US + 40_000L

        assertEquals(clockUs + TunnelClock.VIDEO_DELAY_US, schedule.presentationUs(8_333L, clockUs))
    }

    private companion object {
        const val CLOCK_US = TunnelClock.AUDIO_START_US + 2_000_000L
    }
}
