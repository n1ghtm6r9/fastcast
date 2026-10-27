package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheTunnelClockShowsEachFrameAfterTheAudioReachesItTest {

    @Test
    fun `the first frame is due fifty milliseconds after the audio starts`() {
        val firstFrameUs = 8_000L

        val dueNanos = TunnelClock.videoTimeUs(firstFrameUs) * 1_000L

        assertEquals(50_000_000L, dueNanos - TunnelClock.audioStartNanos(firstFrameUs))
    }

    @Test
    fun `later frames keep their spacing`() {
        assertEquals(8_333L, TunnelClock.videoTimeUs(16_666L) - TunnelClock.videoTimeUs(8_333L))
    }

    @Test
    fun `a stream that starts at zero still gets a timeline far from zero`() {
        assertTrue(TunnelClock.audioStartNanos(0L) >= 1_000_000_000_000_000L)
    }

    @Test
    fun `each silent chunk is ten milliseconds after the one before`() {
        val start = TunnelClock.audioStartNanos(0L)

        assertEquals(start, TunnelClock.chunkNanos(start, 0L))
        assertEquals(start + 30_000_000L, TunnelClock.chunkNanos(start, 3L))
    }

    @Test
    fun `a silent chunk holds ten milliseconds of stereo sixteen bit audio`() {
        assertEquals(480 * 2 * 2, TunnelClock.CHUNK_BYTES)
    }
}
