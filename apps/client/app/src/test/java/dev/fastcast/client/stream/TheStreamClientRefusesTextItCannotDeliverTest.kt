package dev.fastcast.client.stream

import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Test

class TheStreamClientRefusesTextItCannotDeliverTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient(server.url, listener, ReconnectBackoff(listOf(5_000L)))

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `nothing is sent before the client is started`() {
        assertFalse(client.send("early"))
    }

    @Test
    fun `nothing is sent after the client is stopped`() {
        client.start()
        server.accept()

        client.stop()

        assertFalse(client.send("late"))
    }

    @Test
    fun `nothing is sent once the connection is lost`() {
        client.start()
        server.accept().drop()
        listener.nextRetryDelay()

        assertFalse(client.send("lost"))
    }

    @Test
    fun `nothing is sent while a requested reconnect is waiting`() {
        client.start()
        server.accept()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        client.reconnect()

        assertFalse(client.send("between"))
    }
}
