package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheTunnelClockCountsTheAudioThatHasPlayedTest {

    @Test
    fun `the audio timeline starts far from zero`() {
        assertTrue(TunnelClock.AUDIO_START_US >= 1_000_000_000_000L)
        assertEquals(TunnelClock.AUDIO_START_US * 1_000L, TunnelClock.chunkNanos(0L))
    }

    @Test
    fun `each silent chunk is ten milliseconds after the one before`() {
        assertEquals(30_000_000L, TunnelClock.chunkNanos(3L) - TunnelClock.chunkNanos(0L))
    }

    @Test
    fun `a silent chunk holds ten milliseconds of stereo sixteen bit audio`() {
        assertEquals(480 * 2 * 2, TunnelClock.CHUNK_BYTES)
    }

    @Test
    fun `the clock stands at the first chunk before anything has played`() {
        assertEquals(TunnelClock.chunkNanos(0L) / 1_000L, TunnelClock.audioUs(0L, 0L))
    }

    @Test
    fun `the clock reaches each chunk as its frames play`() {
        assertEquals(TunnelClock.chunkNanos(1L) / 1_000L, TunnelClock.audioUs(480L, 0L))
        assertEquals(TunnelClock.AUDIO_START_US + 1_000_000L, TunnelClock.audioUs(48_000L, 0L))
    }

    @Test
    fun `the clock runs on from the moment it was read`() {
        assertEquals(TunnelClock.AUDIO_START_US + 1_002_500L, TunnelClock.audioUs(48_000L, 2_500_000L))
    }
}
