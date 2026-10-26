package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheNativeStreamClientPassesKeyFrameRequestsToItsLinkTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a request reaches the open link`() {
        client.start()
        val handle = link.acceptConnected()

        client.requestKeyFrame()

        assertEquals(listOf(handle), link.keyFrameRequestedOn)
    }

    @Test
    fun `every request reaches the link`() {
        client.start()
        val handle = link.acceptConnected()

        repeat(3) { client.requestKeyFrame() }

        assertEquals(listOf(handle, handle, handle), link.keyFrameRequestedOn)
    }

    @Test
    fun `a request before the client has a link is dropped`() {
        client.requestKeyFrame()

        assertTrue(link.keyFrameRequestedOn.isEmpty())
    }

    @Test
    fun `a request after a reconnect goes to the fresh link only`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)
        client.reconnect()
        val fresh = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        client.requestKeyFrame()

        assertEquals(listOf(fresh), link.keyFrameRequestedOn)
    }

    @Test
    fun `a request after the client is stopped is dropped`() {
        client.start()
        link.acceptConnected()
        client.stop()

        client.requestKeyFrame()

        assertTrue(link.keyFrameRequestedOn.isEmpty())
    }
}
