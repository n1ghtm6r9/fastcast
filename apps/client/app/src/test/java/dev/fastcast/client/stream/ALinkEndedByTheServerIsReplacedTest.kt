package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class ALinkEndedByTheServerIsReplacedTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `an end from the server is reported with its reason`() {
        client.start()
        val handle = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        link.end(handle, "Closed by server")

        assertEquals("Closed by server", listener.nextReason())
    }

    @Test
    fun `the ended link is closed and a new one is opened`() {
        client.start()
        val first = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        link.end(first, "Closed by server")
        listener.nextReason()
        val second = link.accept()

        assertEquals(listOf(first), link.closedHandles)
        assertEquals(2L, second)
    }

    @Test
    fun `an end without a reason is reported as closed by the server`() {
        client.start()
        link.end(link.accept(), "")

        assertEquals("Closed by server", listener.nextReason())
    }

    @Test
    fun `an end with another reason keeps it`() {
        client.start()
        link.end(link.accept(), "Connection reset")

        assertEquals("Connection reset", listener.nextReason())
    }
}
