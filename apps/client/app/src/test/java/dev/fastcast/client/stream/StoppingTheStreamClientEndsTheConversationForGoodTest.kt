package dev.fastcast.client.stream

import org.junit.After
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class StoppingTheStreamClientEndsTheConversationForGoodTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient({ server.url }, listener, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `no new connection is attempted after the stop`() {
        client.start()
        assertNotNull(server.acceptWithin(5_000))

        client.stop()

        assertNull(server.acceptWithin(400))
    }

    @Test
    fun `no new connection is attempted when the stop follows a lost connection`() {
        client.start()
        server.accept().drop()
        listener.nextRetryDelay()

        client.stop()

        assertNull(server.acceptWithin(400))
    }
}
