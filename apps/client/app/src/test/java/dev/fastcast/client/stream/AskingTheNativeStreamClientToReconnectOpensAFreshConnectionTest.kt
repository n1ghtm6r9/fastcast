package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AskingTheNativeStreamClientToReconnectOpensAFreshConnectionTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a second connection is accepted after the requested reconnect`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        client.reconnect()
        link.frame(link.accept(), frameBytes(true, 7L, byteArrayOf(1)))

        assertEquals(7L, listener.nextFrame().timestampMs)
    }

    @Test
    fun `a requested reconnect is not reported as a disconnection`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        client.reconnect()
        link.acceptConnected()

        assertEquals(
            listOf(RecordingStreamListener.CONNECTING, RecordingStreamListener.CONNECTED),
            List(2) { listener.nextEvent() },
        )
        assertTrue(listener.hasNoEventWithin(300L))
    }
}
