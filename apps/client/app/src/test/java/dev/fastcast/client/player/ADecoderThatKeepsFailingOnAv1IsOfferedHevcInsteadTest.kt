package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Test

class ADecoderThatKeepsFailingOnAv1IsOfferedHevcInsteadTest {

    private val offer = StreamOffer(
        ADDRESS,
        StreamRequest(WIDTH, HEIGHT, FPS, listOf("av1", "hevc", "avc"), setOf("av1", "hevc")),
    )

    @Test
    fun `two ten bit av1 failures drop ten bits from av1 only`() {
        repeat(2) { av1Fails() }

        assertEquals(setOf("hevc"), tenBitOnOffer())
        assertEquals(listOf("av1", "hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `two more av1 failures drop av1 and keep ten bit hevc`() {
        repeat(4) { av1Fails() }

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
        assertEquals(setOf("hevc"), tenBitOnOffer())
    }

    private fun codecsOnOffer(): List<String>? = offer.target().request?.codecs

    private fun tenBitOnOffer(): Set<String>? = offer.target().request?.tenBitCodecs

    private fun av1Fails() {
        offer.codecConfigured(AV1_CODEC)
        offer.decoderFailed()
    }

    private companion object {
        const val ADDRESS = "192.168.0.10:3278"
        const val WIDTH = 3840
        const val HEIGHT = 2160
        const val FPS = 120
        const val AV1_CODEC = "av01.0.14M.10"
    }
}
