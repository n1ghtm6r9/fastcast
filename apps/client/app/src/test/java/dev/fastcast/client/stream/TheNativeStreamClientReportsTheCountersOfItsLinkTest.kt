package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheNativeStreamClientReportsTheCountersOfItsLinkTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `no socket drops are known before the client has a link`() {
        assertNull(client.socketDrops())
    }

    @Test
    fun `the socket drops of the open link are reported`() {
        client.start()
        val handle = link.accept()

        link.dropOnSocket(handle, 417)

        assertEquals(417, client.socketDrops())
    }

    @Test
    fun `socket drops the link cannot count are not known`() {
        client.start()
        link.accept()

        assertNull(client.socketDrops())
    }

    @Test
    fun `no pause is known before the client has a link`() {
        assertNull(client.takeLongestPauseMs())
    }

    @Test
    fun `the longest pause of the open link is handed over once`() {
        client.start()
        val handle = link.accept()
        link.pauseFor(handle, 140)

        assertEquals(140, client.takeLongestPauseMs())
        assertEquals(0, client.takeLongestPauseMs())
    }

    @Test
    fun `no counters are known once the client is stopped`() {
        client.start()
        val handle = link.accept()
        link.dropOnSocket(handle, 417)
        link.pauseFor(handle, 140)

        client.stop()

        assertNull(client.socketDrops())
        assertNull(client.takeLongestPauseMs())
    }
}
