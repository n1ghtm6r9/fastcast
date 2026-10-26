package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotEquals
import org.junit.Test

class TheNativeStreamClientReportsTheReceiveBufferOfItsLinkTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `no buffer is known before the client has a link`() {
        assertEquals(0, client.receiveBufferBytes())
    }

    @Test
    fun `the buffer of the open link is reported`() {
        client.start()
        val handle = link.accept()

        assertEquals(link.receiveBufferOf(handle), client.receiveBufferBytes())
    }

    @Test
    fun `a reconnect reports the buffer of the fresh link`() {
        client.start()
        val first = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)
        client.reconnect()
        val fresh = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        assertEquals(link.receiveBufferOf(fresh), client.receiveBufferBytes())
        assertNotEquals(link.receiveBufferOf(first), client.receiveBufferBytes())
    }

    @Test
    fun `no buffer is known once the client is stopped`() {
        client.start()
        link.accept()

        client.stop()

        assertEquals(0, client.receiveBufferBytes())
    }
}
