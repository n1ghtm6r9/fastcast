package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheAudioClockRunsOnBetweenSparseReadingsTest {

    private val sampler = AudioClockSampler(startedNanos = START)
    private var polls = 0

    private fun reading(played: PlayedAudio?): () -> PlayedAudio? = {
        polls++
        played
    }

    @Test
    fun `audio that has not played yet gives no clock`() {
        assertNull(sampler.clockUs(START, reading(null)))
        assertNull(sampler.clockUs(START + 1_000_000L, reading(PlayedAudio(0L, START))))
    }

    @Test
    fun `the audio is read on every call until it plays`() {
        sampler.clockUs(START, reading(null))
        sampler.clockUs(START + 1_000_000L, reading(PlayedAudio(0L, START)))
        sampler.clockUs(START + 2_000_000L, reading(null))

        assertEquals(3, polls)
    }

    @Test
    fun `the clock runs on from the last reading`() {
        val readAt = START + 300_000_000L
        sampler.clockUs(readAt, reading(PlayedAudio(48_000L, readAt)))

        val clockUs = sampler.clockUs(readAt + 50_000_000L, reading(PlayedAudio(999_999L, readAt)))

        assertEquals(TunnelClock.audioUs(48_000L, 50_000_000L), clockUs)
        assertEquals(1, polls)
    }

    @Test
    fun `an old reading is read again`() {
        val readAt = START + 300_000_000L
        sampler.clockUs(readAt, reading(PlayedAudio(48_000L, readAt)))
        val later = readAt + AudioClockSampler.POLL_INTERVAL_NANOS

        val clockUs = sampler.clockUs(later, reading(PlayedAudio(57_600L, later)))

        assertEquals(TunnelClock.audioUs(57_600L, 0L), clockUs)
        assertEquals(2, polls)
    }

    @Test
    fun `a failed reading keeps the clock running from the last one`() {
        val readAt = START + 300_000_000L
        sampler.clockUs(readAt, reading(PlayedAudio(48_000L, readAt)))
        val later = readAt + AudioClockSampler.POLL_INTERVAL_NANOS

        val clockUs = sampler.clockUs(later, reading(null))

        assertEquals(TunnelClock.audioUs(48_000L, AudioClockSampler.POLL_INTERVAL_NANOS), clockUs)
    }

    private companion object {
        const val START = 5_000_000_000L
    }
}
