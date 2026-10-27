package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheAudioClockTellsWhenItStartedAndHowFastItRunsTest {

    private val sampler = AudioClockSampler(startedNanos = START)

    @Test
    fun `nothing is known before the audio plays`() {
        sampler.clockUs(START + 10_000_000L) { null }

        assertNull(sampler.movedAfterNanos)
        assertNull(sampler.rate())
    }

    @Test
    fun `the start is counted back from the first frames that played`() {
        sampler.clockUs(START + 100_000_000L) { PlayedAudio(0L, START + 100_000_000L) }
        sampler.clockUs(START + 180_000_000L) { PlayedAudio(480L, START + 175_000_000L) }

        assertEquals(165_000_000L, sampler.movedAfterNanos)
    }

    @Test
    fun `the rate compares the audio played with the time it took`() {
        sampler.clockUs(START + 1_000_000_000L) { PlayedAudio(48_000L, START + 1_000_000_000L) }
        sampler.clockUs(START + 3_000_000_000L) { PlayedAudio(144_096L, START + 3_000_000_000L) }

        assertEquals(1.001, requireNotNull(sampler.rate()), 1e-9)
    }

    @Test
    fun `one reading gives no rate`() {
        sampler.clockUs(START + 1_000_000_000L) { PlayedAudio(48_000L, START + 1_000_000_000L) }

        assertNull(sampler.rate())
    }

    private companion object {
        const val START = 5_000_000_000L
    }
}
