package dev.fastcast.client.player

import dev.fastcast.client.stream.FakeStreamLink
import dev.fastcast.client.stream.NativeStreamClient
import dev.fastcast.client.stream.ReconnectBackoff
import dev.fastcast.client.stream.RecordingStreamListener
import dev.fastcast.client.stream.StreamRequest
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheServerIsAskedForAvcOnceHevcKeepsFailingTest {

    private val link = FakeStreamLink()
    private val offer = StreamOffer("192.168.0.10:3278", StreamRequest(3840, 2160, 60, listOf("hevc", "avc")))
    private val client = NativeStreamClient(offer::target, RecordingStreamListener(), link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `the connection after two failed hevc attempts asks for avc only`() {
        client.start()
        link.accept()
        decoderFailsOnHevc()
        link.accept()
        decoderFailsOnHevc()
        link.accept()

        assertEquals(
            listOf(BOTH_CODECS, BOTH_CODECS, AVC_ONLY),
            link.targets.map { it.request?.codecs },
        )
    }

    private fun decoderFailsOnHevc() {
        offer.codecConfigured("hvc1.1.6.L153.90")
        offer.decoderFailed()
        client.reconnect()
    }

    private companion object {
        val BOTH_CODECS = listOf("hevc", "avc")
        val AVC_ONLY = listOf("avc")
    }
}
