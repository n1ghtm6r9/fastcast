package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Test

class HevcFailuresCountWhileAv1LeadsTheOfferTest {

    private val offer = StreamOffer(
        ADDRESS,
        StreamRequest(WIDTH, HEIGHT, FPS, listOf("av1", "hevc", "avc"), setOf("av1", "hevc")),
    )

    @Test
    fun `two ten bit hevc failures drop ten bits from hevc only`() {
        repeat(2) { fails(HEVC_CODEC) }

        assertEquals(setOf("av1"), tenBitOnOffer())
        assertEquals(listOf("av1", "hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `two more hevc failures drop hevc and keep av1`() {
        repeat(4) { fails(HEVC_CODEC) }

        assertEquals(listOf("av1", "avc"), codecsOnOffer())
        assertEquals(setOf("av1"), tenBitOnOffer())
    }

    @Test
    fun `a failing avc fallback leaves the offer alone`() {
        repeat(4) { fails(AVC_CODEC) }

        assertEquals(listOf("av1", "hevc", "avc"), codecsOnOffer())
        assertEquals(setOf("av1", "hevc"), tenBitOnOffer())
    }

    @Test
    fun `the last codec apart from av1 is never dropped`() {
        repeat(4) { fails(HEVC_CODEC) }
        repeat(5) { fails(AVC_CODEC) }

        assertEquals(listOf("av1", "avc"), codecsOnOffer())
    }

    @Test
    fun `a late av1 failure after av1 was dropped does not count against hevc`() {
        repeat(4) { fails(AV1_CODEC) }
        fails(AV1_CODEC)
        fails(HEVC_CODEC)

        assertEquals(setOf("hevc"), tenBitOnOffer())
    }

    private fun codecsOnOffer(): List<String>? = offer.target().request?.codecs

    private fun tenBitOnOffer(): Set<String>? = offer.target().request?.tenBitCodecs

    private fun fails(codec: String) {
        offer.codecConfigured(codec)
        offer.decoderFailed()
    }

    private companion object {
        const val ADDRESS = "192.168.0.10:3278"
        const val WIDTH = 3840
        const val HEIGHT = 2160
        const val FPS = 120
        const val AV1_CODEC = "av01.0.14M.10"
        const val HEVC_CODEC = "hvc1.2.4.L156.90"
        const val AVC_CODEC = "avc1.640033"
    }
}
