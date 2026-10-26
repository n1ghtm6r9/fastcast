package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNativeStreamClientClosesEveryLinkItRetiresTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a requested reconnect closes the old link once`() {
        client.start()
        val first = link.accept()

        client.reconnect()
        link.accept()

        assertEquals(listOf(first), link.closedHandles)
    }

    @Test
    fun `stopping the client closes its link once`() {
        client.start()
        val handle = link.accept()

        client.stop()
        client.stop()

        assertEquals(listOf(handle), link.closedHandles)
    }
}
