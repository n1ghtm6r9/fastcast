package dev.fastcast.client.stream

import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheServerSeesTheRequestedDisplayModeOnTheWireTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private var client: StreamClient? = null

    @After
    fun tearDown() {
        client?.stop()
        server.close()
    }

    private fun connectWith(request: StreamRequest?): FakeConnection {
        val started = StreamClient({ StreamUrl.build(server.address, request) }, listener, ReconnectBackoff(listOf(30L)))
        client = started
        started.start()
        return server.accept()
    }

    @Test
    fun `the request line carries the display mode and the codecs`() {
        val connection = connectWith(StreamRequest(3840, 2160, 60, listOf("hevc", "avc")))

        assertEquals("/ws?width=3840&height=2160&fps=60&codecs=hevc,avc", connection.requestTarget)
    }

    @Test
    fun `the request line carries no codecs when none qualifies`() {
        val connection = connectWith(StreamRequest(1920, 1080, 30, emptyList()))

        assertEquals("/ws?width=1920&height=1080&fps=30", connection.requestTarget)
    }

    @Test
    fun `an old style request line has no query`() {
        val connection = connectWith(null)

        assertEquals("/ws", connection.requestTarget)
    }
}
