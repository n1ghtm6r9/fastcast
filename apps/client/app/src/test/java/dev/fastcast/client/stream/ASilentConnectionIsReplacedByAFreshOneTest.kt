package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ASilentConnectionIsReplacedByAFreshOneTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient(server.url, listener, ReconnectBackoff(listOf(30L)), SILENCE_LIMIT_MS)

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `a connection that stays silent past the limit is reopened`() {
        client.start()
        server.accept()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        listener.skipTo(RecordingStreamListener.DISCONNECTED)
        server.accept().sendBinary(frameBytes(true, 9L, byteArrayOf(1)))

        assertEquals(9L, listener.nextFrame().timestampMs)
    }

    @Test
    fun `a connection that keeps delivering frames is kept`() {
        client.start()
        val connection = server.accept()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        repeat(FRAMES) { index ->
            connection.sendBinary(frameBytes(index == 0, index.toLong(), byteArrayOf(1)))
            Thread.sleep(FRAME_GAP_MS)
        }

        assertNull(server.acceptWithin(NO_RECONNECT_WAIT_MS))
    }

    private companion object {
        const val SILENCE_LIMIT_MS = 300L
        const val FRAMES = 20
        const val FRAME_GAP_MS = 50L
        const val NO_RECONNECT_WAIT_MS = 200
    }
}
