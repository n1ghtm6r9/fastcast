package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheStreamClientReconnectsWithGrowingDelaysTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient(server.url, listener, ReconnectBackoff(listOf(40L, 80L, 160L)))

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `each lost connection is followed by a longer wait until the cap`() {
        client.start()

        val delays = List(4) {
            server.accept().drop()
            listener.nextRetryDelay()
        }

        assertEquals(listOf(40L, 80L, 160L, 160L), delays)
    }

    @Test
    fun `a delivered frame restarts the delays from the first one`() {
        client.start()
        server.accept().drop()
        val firstDelay = listener.nextRetryDelay()
        server.accept().drop()
        val secondDelay = listener.nextRetryDelay()

        val connection = server.accept()
        connection.sendBinary(frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        connection.drop()
        val delayAfterFrame = listener.nextRetryDelay()

        assertEquals(listOf(40L, 80L, 40L), listOf(firstDelay, secondDelay, delayAfterFrame))
    }

    @Test
    fun `every new attempt is announced as connecting`() {
        client.start()
        server.accept().drop()
        server.accept().drop()
        server.accept()

        repeat(3) { listener.skipTo(RecordingStreamListener.CONNECTING) }
    }
}
