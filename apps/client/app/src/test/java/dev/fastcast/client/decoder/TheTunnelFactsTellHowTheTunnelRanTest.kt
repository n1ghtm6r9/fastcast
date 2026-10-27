package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelFactsTellHowTheTunnelRanTest {

    @Test
    fun `the facts name the decoder, its first frame, how often it started, the audio and the video timing`() {
        val stats = DecoderStats().apply {
            decoderName = "c2.mtk.hevc.decoder"
            tunnelFrameReady = true
            codecStarts.set(2L)
        }

        assertEquals(
            listOf(
                "tunnel decoder c2.mtk.hevc.decoder",
                "first tunnel frame ready",
                "codec started 2 times",
                "audio clock",
                "video anchors",
            ),
            TunnelFacts.of(stats, "audio clock", "video anchors"),
        )
    }

    @Test
    fun `a tunnel that never started says so`() {
        assertEquals(
            listOf("tunnel decoder none", "first tunnel frame not reported", "codec started 0 times", "audio", "video"),
            TunnelFacts.of(DecoderStats(), "audio", "video"),
        )
    }
}
