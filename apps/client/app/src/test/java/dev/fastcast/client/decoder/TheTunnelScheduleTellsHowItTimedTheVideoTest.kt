package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheTunnelScheduleTellsHowItTimedTheVideoTest {

    private val schedule = TunnelSchedule()

    @Test
    fun `a fresh schedule has timed nothing`() {
        assertEquals("video anchors 0 (0 on the running audio clock), retimes 0, slack 50.0 ms", schedule.describe())
    }

    @Test
    fun `a schedule with a shorter delay starts its slack there`() {
        assertEquals("video anchors 0 (0 on the running audio clock), retimes 0, slack 12.0 ms", TunnelSchedule(12_000L).describe())
    }

    @Test
    fun `the report counts the anchors made on the running clock`() {
        schedule.presentationUs(0L, null)
        schedule.presentationUs(8_333L, CLOCK_US)

        assertEquals("video anchors 2 (1 on the running audio clock), retimes 0, slack 50.0 ms", schedule.describe())
    }

    @Test
    fun `the report counts the retimes`() {
        schedule.presentationUs(0L, CLOCK_US)
        for (index in 1..600) {
            val frameUs = index * 8_333L
            schedule.presentationUs(frameUs, CLOCK_US + frameUs + 40_000L)
        }

        val report = schedule.describe()

        assertTrue(report, report.startsWith("video anchors 1 (1 on the running audio clock), retimes 1, slack "))
    }

    private companion object {
        const val CLOCK_US = TunnelClock.AUDIO_START_US + 2_000_000L
    }
}
