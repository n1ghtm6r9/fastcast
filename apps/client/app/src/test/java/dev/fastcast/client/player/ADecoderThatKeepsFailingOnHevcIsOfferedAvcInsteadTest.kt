package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ADecoderThatKeepsFailingOnHevcIsOfferedAvcInsteadTest {

    private val offer = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc", "avc")))

    @Test
    fun `every supported codec is offered at first`() {
        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `a single hevc failure keeps hevc on offer`() {
        hevcFails()

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `two hevc failures in a row drop hevc from the next target`() {
        hevcFails()
        hevcFails()

        assertEquals(listOf("avc"), codecsOnOffer())
    }

    @Test
    fun `a healthy stream between two failures forgives the first one`() {
        hevcFails()
        offer.streamHealthy()
        hevcFails()

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `a dropped codec stays dropped when a stream is healthy afterwards`() {
        hevcFails()
        hevcFails()
        offer.streamHealthy()

        assertEquals(listOf("avc"), codecsOnOffer())
    }

    @Test
    fun `the last codec on offer is never dropped`() {
        hevcFails()
        hevcFails()
        repeat(5) { avcFails() }

        assertEquals(listOf("avc"), codecsOnOffer())
    }

    @Test
    fun `a failing fallback codec leaves the offer alone`() {
        repeat(4) { avcFails() }

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `a failure before any codec was configured is ignored`() {
        offer.decoderFailed()
        offer.decoderFailed()

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `a codec the offer does not know is ignored`() {
        repeat(2) {
            offer.codecConfigured("vp09.00.10.08")
            offer.decoderFailed()
        }

        assertEquals(listOf("hevc", "avc"), codecsOnOffer())
    }

    @Test
    fun `an offer without a display request asks for no mode`() {
        val plain = StreamOffer(ADDRESS, null)

        assertNull(plain.target().request)
    }

    @Test
    fun `an offer without a decodable codec lists none`() {
        val none = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, emptyList()))
        none.codecConfigured(HEVC_CODEC)
        none.decoderFailed()

        val request = requireNotNull(none.target().request)
        assertEquals(listOf(WIDTH, HEIGHT, FPS), listOf(request.width, request.height, request.fps))
        assertEquals(emptyList<String>(), request.codecs)
    }

    @Test
    fun `dropping a codec keeps the requested mode`() {
        hevcFails()
        hevcFails()

        val request = requireNotNull(offer.target().request)
        assertEquals(listOf(WIDTH, HEIGHT, FPS), listOf(request.width, request.height, request.fps))
    }

    private fun codecsOnOffer(): List<String>? = offer.target().request?.codecs

    private fun hevcFails() {
        offer.codecConfigured(HEVC_CODEC)
        offer.decoderFailed()
    }

    private fun avcFails() {
        offer.codecConfigured(AVC_CODEC)
        offer.decoderFailed()
    }

    private companion object {
        const val ADDRESS = "192.168.0.10:3278"
        const val WIDTH = 3840
        const val HEIGHT = 2160
        const val FPS = 60
        const val HEVC_CODEC = "hvc1.1.6.L153.90"
        const val AVC_CODEC = "avc1.640033"
    }
}
