package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertFalse
import org.junit.Test

class TheNativeStreamClientRefusesTextItCannotDeliverTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(5_000L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `nothing is sent before the client is started`() {
        assertFalse(client.send("early"))
    }

    @Test
    fun `nothing is sent after the client is stopped`() {
        client.start()
        link.accept()

        client.stop()

        assertFalse(client.send("late"))
    }

    @Test
    fun `nothing is sent once the connection is lost`() {
        client.start()
        link.end(link.accept(), "")
        listener.nextRetryDelay()

        assertFalse(client.send("lost"))
    }

    @Test
    fun `nothing is sent while a requested reconnect is waiting`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        client.reconnect()

        assertFalse(client.send("between"))
    }
}
