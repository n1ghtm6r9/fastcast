package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheSurfaceFactsTellHowTheUsualPathRanTest {

    @Test
    fun `the facts name the decoder, how often it started and how long frames took to decode and to be shown`() {
        val stats = DecoderStats().apply {
            decoderName = "c2.mtk.av1.decoder"
            codecStarts.set(1L)
        }
        stats.frameQueued(1_000L, 0L)
        stats.frameDecoded(1_000L, 6_000_000L)
        stats.framePresented(1_000L, 30_000_000L)

        assertEquals(
            listOf(
                "surface decoder c2.mtk.av1.decoder",
                "codec started 1 times",
                "frames decoded 6.0 ms after queueing",
                "frames shown 30.0 ms after queueing",
                "shown after queueing per second: none",
            ),
            SurfaceFacts.of(stats),
        )
    }

    @Test
    fun `every second of shown frames tells how long they waited`() {
        val stats = DecoderStats()
        repeat(2 * FRAMES_PER_SECOND) { index ->
            val ptsUs = index.toLong()
            val waitedMs = if (index < FRAMES_PER_SECOND) 48L else 40L
            stats.frameQueued(ptsUs, 0L)
            stats.framePresented(ptsUs, waitedMs * 1_000_000L)
        }

        assertEquals("shown after queueing per second: 48 40 ms", SurfaceFacts.of(stats)[4])
    }

    @Test
    fun `a decoder that never started says so`() {
        assertEquals(
            listOf(
                "surface decoder none",
                "codec started 0 times",
                "no frame decoded after queueing",
                "no frame shown after queueing",
                "shown after queueing per second: none",
            ),
            SurfaceFacts.of(DecoderStats()),
        )
    }

    private companion object {
        const val FRAMES_PER_SECOND = 120
    }
}
