package dev.fastcast.client.player

import dev.fastcast.client.stream.StreamRequest
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheOfferTargetsTheServerWithTheRequestedModeAndCodecsTest {

    private val offer = StreamOffer("192.168.0.10:3278", StreamRequest(3840, 2160, 120, listOf("hevc", "avc")))

    @Test
    fun `the target is the host and the port of the address`() {
        val target = offer.target()

        assertEquals("192.168.0.10", target.host)
        assertEquals(3278, target.port)
    }

    @Test
    fun `the target carries the requested mode and codecs`() {
        val request = requireNotNull(offer.target().request)

        assertEquals(3840, request.width)
        assertEquals(2160, request.height)
        assertEquals(120, request.fps)
        assertEquals(listOf("hevc", "avc"), request.codecs)
    }

    @Test
    fun `an address without a port gets the default port`() {
        assertEquals(3278, StreamOffer("192.168.0.10", null).target().port)
    }

    @Test
    fun `an address without a port keeps its whole text as the host`() {
        assertEquals("192.168.0.10", StreamOffer("192.168.0.10", null).target().host)
    }

    @Test
    fun `a port that is not a number gets the default port`() {
        val target = StreamOffer("192.168.0.10:abc", null).target()

        assertEquals("192.168.0.10", target.host)
        assertEquals(3278, target.port)
    }

    @Test
    fun `a port outside the valid range gets the default port`() {
        assertEquals(3278, StreamOffer("192.168.0.10:0", null).target().port)
        assertEquals(3278, StreamOffer("192.168.0.10:65536", null).target().port)
    }

    @Test
    fun `another valid port is kept`() {
        assertEquals(4000, StreamOffer("10.0.0.5:4000", null).target().port)
    }

    @Test
    fun `an offer without a request targets the server without a mode`() {
        assertNull(StreamOffer("192.168.0.10:3278", null).target().request)
    }
}
