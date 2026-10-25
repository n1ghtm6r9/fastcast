package dev.fastcast.client.stream

import java.net.URI
import okhttp3.Request
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheStreamUrlCarriesTheDisplayModeAndTheCodecsTest {

    private val address = "192.168.0.10:3278"

    @Test
    fun `the query lists width height and fps and then the codecs best first`() {
        val url = StreamUrl.build(address, StreamRequest(3840, 2160, 60, listOf("hevc", "avc")))

        assertEquals("ws://192.168.0.10:3278/ws?width=3840&height=2160&fps=60&codecs=hevc,avc", url)
    }

    @Test
    fun `a single codec is listed alone`() {
        val url = StreamUrl.build(address, StreamRequest(1920, 1080, 30, listOf("avc")))

        assertEquals("ws://192.168.0.10:3278/ws?width=1920&height=1080&fps=30&codecs=avc", url)
    }

    @Test
    fun `the codecs parameter is left out when no codec qualifies`() {
        val url = StreamUrl.build(address, StreamRequest(3840, 2160, 60, emptyList()))

        assertEquals("ws://192.168.0.10:3278/ws?width=3840&height=2160&fps=60", url)
    }

    @Test
    fun `without a request the url is the plain endpoint an old server expects`() {
        assertEquals("ws://192.168.0.10:3278/ws", StreamUrl.build(address, null))
    }

    @Test
    fun `the url is a valid websocket uri`() {
        val uri = URI(StreamUrl.build(address, StreamRequest(3840, 2160, 60, listOf("hevc", "avc"))))

        assertEquals("ws", uri.scheme)
        assertEquals("192.168.0.10", uri.host)
        assertEquals(3278, uri.port)
        assertEquals("/ws", uri.path)
        assertEquals("width=3840&height=2160&fps=60&codecs=hevc,avc", uri.rawQuery)
    }

    @Test
    fun `the http client reads every parameter back unchanged`() {
        val url = Request.Builder()
            .url(StreamUrl.build(address, StreamRequest(3840, 2160, 60, listOf("hevc", "avc"))))
            .build()
            .url

        assertEquals("3840", url.queryParameter("width"))
        assertEquals("2160", url.queryParameter("height"))
        assertEquals("60", url.queryParameter("fps"))
        assertEquals("hevc,avc", url.queryParameter("codecs"))
    }

    @Test
    fun `the http client sees no codecs parameter when none was requested`() {
        val url = Request.Builder()
            .url(StreamUrl.build(address, StreamRequest(3840, 2160, 60, emptyList())))
            .build()
            .url

        assertNull(url.queryParameter("codecs"))
    }
}
