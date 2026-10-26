package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Test

class ADecoderThatKeepsFailingOnTenBitHevcFallsBackToEightBitsFirstTest {

    private val offer = StreamOffer(
        ADDRESS,
        StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc", "avc"), setOf("hevc")),
    )

    @Test
    fun `ten bit hevc is offered at first`() {
        assertEquals(setOf("hevc"), tenBitOnOffer())
    }

    @Test
    fun `a single ten bit hevc failure keeps ten bits on offer`() {
        hevcFails()

        assertEquals(setOf("hevc"), tenBitOnOffer())
        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `two ten bit hevc failures drop ten bits but keep hevc`() {
        hevcFails()
        hevcFails()

        assertEquals(emptySet<String>(), tenBitOnOffer())
        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `two more hevc failures then drop hevc`() {
        repeat(4) { hevcFails() }

        assertEquals(listOf("avc"), codecsOnOffer())
    }

    @Test
    fun `a healthy stream between two ten bit failures forgives the first one`() {
        hevcFails()
        offer.streamHealthy()
        hevcFails()

        assertEquals(setOf("hevc"), tenBitOnOffer())
    }

    @Test
    fun `a failing fallback codec leaves the ten bit offer alone`() {
        repeat(4) { avcFails() }

        assertEquals(setOf("hevc"), tenBitOnOffer())
        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `a lone hevc still falls back from ten to eight bits`() {
        val lone = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc"), setOf("hevc")))

        repeat(2) { fails(lone, HEVC_CODEC) }

        val request = requireNotNull(lone.target().request)
        assertEquals(listOf("hevc"), request.codecs)
        assertEquals(emptySet<String>(), request.tenBitCodecs)
    }

    @Test
    fun `a lone hevc is never dropped after it fell back to eight bits`() {
        val lone = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc"), setOf("hevc")))

        repeat(6) { fails(lone, HEVC_CODEC) }

        assertEquals(listOf("hevc"), lone.target().request?.codecs)
    }

    @Test
    fun `an offer without ten bit codecs lists none`() {
        val eightBit = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc", "avc")))

        assertEquals(emptySet<String>(), eightBit.target().request?.tenBitCodecs)
    }

    private fun codecsOnOffer(): List<String>? = offer.target().request?.codecs

    private fun tenBitOnOffer(): Set<String>? = offer.target().request?.tenBitCodecs

    private fun hevcFails() = fails(offer, HEVC_CODEC)

    private fun avcFails() = fails(offer, AVC_CODEC)

    private fun fails(on: StreamOffer, codec: String) {
        on.codecConfigured(codec)
        on.decoderFailed()
    }

    private companion object {
        const val ADDRESS = "192.168.0.10:3278"
        const val WIDTH = 3840
        const val HEIGHT = 2160
        const val FPS = 120
        const val HEVC_CODEC = "hvc1.2.4.L156.90"
        const val AVC_CODEC = "avc1.640033"
    }
}
