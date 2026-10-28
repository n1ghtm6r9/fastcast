package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelFactsTellHowTheTunnelRanTest {

    @Test
    fun `the facts name the decoder, its first frame, how often it started, how late it showed frames and the timing`() {
        val stats = DecoderStats().apply {
            decoderName = "c2.mtk.hevc.decoder"
            tunnelFrameReady = true
            codecStarts.set(2L)
        }
        stats.frameQueued(1_000L, 0L)
        stats.framePresented(1_000L, 25_000_000L)
        listOf(3_000L, 1_000L, 7_000L).forEach(stats::frameLate)

        assertEquals(
            listOf(
                "tunnel decoder c2.mtk.hevc.decoder",
                "first tunnel frame ready",
                "codec started 2 times",
                "frames shown 25.0 ms after queueing and 3.0 ms after their time, 90% within 7.0 ms",
                "after queueing per second: none; after their time per second: none; 0 frames decoded only",
                "audio clock",
                "video anchors",
            ),
            TunnelFacts.of(stats, "audio clock", "video anchors"),
        )
    }

    @Test
    fun `every second of shown frames tells how long they waited and how late they were`() {
        val stats = DecoderStats()
        repeat(2 * FRAMES_PER_SECOND) { index ->
            val ptsUs = index.toLong()
            val waitedMs = if (index < FRAMES_PER_SECOND) 36L else 28L
            stats.frameQueued(ptsUs, 0L)
            stats.framePresented(ptsUs, waitedMs * 1_000_000L)
            stats.frameLate((waitedMs - 14L) * 1_000L)
        }
        stats.framesDecodedOnly.set(1L)

        assertEquals(
            "after queueing per second: 36 28 ms; after their time per second: 22 14 ms; 1 frames decoded only",
            TunnelFacts.of(stats, "audio", "video")[4],
        )
    }

    @Test
    fun `a tunnel that never started says so`() {
        assertEquals(
            listOf(
                "tunnel decoder none",
                "first tunnel frame not reported",
                "codec started 0 times",
                "no frame shown after queueing, no frame timed",
                "after queueing per second: none; after their time per second: none; 0 frames decoded only",
                "audio",
                "video",
            ),
            TunnelFacts.of(DecoderStats(), "audio", "video"),
        )
    }

    private companion object {
        const val FRAMES_PER_SECOND = 120
    }
}
