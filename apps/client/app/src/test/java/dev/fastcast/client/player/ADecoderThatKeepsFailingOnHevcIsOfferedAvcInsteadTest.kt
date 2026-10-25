package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Test

class ADecoderThatKeepsFailingOnHevcIsOfferedAvcInsteadTest {

    private val offer = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, listOf("hevc", "avc")))

    @Test
    fun `every supported codec is offered at first`() {
        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `a single hevc failure keeps hevc on offer`() {
        hevcFails()

        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `two hevc failures in a row drop hevc from the next url`() {
        hevcFails()
        hevcFails()

        assertEquals(urlWithCodecs("avc"), offer.url())
    }

    @Test
    fun `a healthy stream between two failures forgives the first one`() {
        hevcFails()
        offer.streamHealthy()
        hevcFails()

        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `a dropped codec stays dropped when a stream is healthy afterwards`() {
        hevcFails()
        hevcFails()
        offer.streamHealthy()

        assertEquals(urlWithCodecs("avc"), offer.url())
    }

    @Test
    fun `the last codec on offer is never dropped`() {
        hevcFails()
        hevcFails()
        repeat(5) { avcFails() }

        assertEquals(urlWithCodecs("avc"), offer.url())
    }

    @Test
    fun `a failing fallback codec leaves the offer alone`() {
        repeat(4) { avcFails() }

        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `a failure before any codec was configured is ignored`() {
        offer.decoderFailed()
        offer.decoderFailed()

        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `a codec the offer does not know is ignored`() {
        repeat(2) {
            offer.codecConfigured("vp09.00.10.08")
            offer.decoderFailed()
        }

        assertEquals(urlWithCodecs("hevc,avc"), offer.url())
    }

    @Test
    fun `an offer without a display request is the plain endpoint`() {
        val plain = StreamOffer(ADDRESS, null)

        assertEquals("ws://$ADDRESS/ws", plain.url())
    }

    @Test
    fun `an offer without a decodable codec lists none`() {
        val none = StreamOffer(ADDRESS, StreamRequest(WIDTH, HEIGHT, FPS, emptyList()))
        none.codecConfigured(HEVC_CODEC)
        none.decoderFailed()

        assertEquals("ws://$ADDRESS/ws?width=$WIDTH&height=$HEIGHT&fps=$FPS", none.url())
    }

    private fun hevcFails() {
        offer.codecConfigured(HEVC_CODEC)
        offer.decoderFailed()
    }

    private fun avcFails() {
        offer.codecConfigured(AVC_CODEC)
        offer.decoderFailed()
    }

    private fun urlWithCodecs(codecs: String): String =
        "ws://$ADDRESS/ws?width=$WIDTH&height=$HEIGHT&fps=$FPS&codecs=$codecs"

    private companion object {
        const val ADDRESS = "192.168.0.10:3278"
        const val WIDTH = 3840
        const val HEIGHT = 2160
        const val FPS = 60
        const val HEVC_CODEC = "hvc1.1.6.L153.90"
        const val AVC_CODEC = "avc1.640033"
    }
}
