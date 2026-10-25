package dev.fastcast.client.player

import dev.fastcast.client.stream.FakeWebSocketServer
import dev.fastcast.client.stream.ReconnectBackoff
import dev.fastcast.client.stream.RecordingStreamListener
import dev.fastcast.client.stream.StreamClient
import dev.fastcast.client.stream.StreamRequest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheServerIsAskedForAvcOnceHevcKeepsFailingTest {

    private val server = FakeWebSocketServer()
    private val offer = StreamOffer(server.address, StreamRequest(3840, 2160, 60, listOf("hevc", "avc")))
    private val client = StreamClient(offer::url, RecordingStreamListener(), ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `the connection after two failed hevc attempts asks for avc only`() {
        client.start()
        val first = server.accept()
        decoderFailsOnHevc()
        val second = server.accept()
        decoderFailsOnHevc()
        val third = server.accept()

        assertEquals(
            listOf(BOTH_CODECS, BOTH_CODECS, AVC_ONLY),
            listOf(first.requestTarget, second.requestTarget, third.requestTarget),
        )
    }

    private fun decoderFailsOnHevc() {
        offer.codecConfigured("hvc1.1.6.L153.90")
        offer.decoderFailed()
        client.reconnect()
    }

    private companion object {
        const val BOTH_CODECS = "/ws?width=3840&height=2160&fps=60&codecs=hevc,avc"
        const val AVC_ONLY = "/ws?width=3840&height=2160&fps=60&codecs=avc"
    }
}
