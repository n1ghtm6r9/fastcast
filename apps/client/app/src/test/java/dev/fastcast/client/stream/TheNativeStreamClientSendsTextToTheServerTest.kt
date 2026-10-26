package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheNativeStreamClientSendsTextToTheServerTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    private fun connect(): Long {
        client.start()
        val handle = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)
        return handle
    }

    @Test
    fun `a short text reaches the server unchanged`() {
        val handle = connect()

        assertTrue(client.send("""{"type":"stats"}"""))

        assertEquals("""{"type":"stats"}""", link.nextStats(handle))
    }

    @Test
    fun `a text longer than one hundred twenty five bytes reaches the server unchanged`() {
        val handle = connect()
        val text = "stats ".repeat(50)

        assertTrue(client.send(text))

        assertEquals(text, link.nextStats(handle))
    }

    @Test
    fun `text outside ascii reaches the server unchanged`() {
        val handle = connect()
        val text = "декодер c2.mtk.hevc.decoder ☃"

        assertTrue(client.send(text))

        assertEquals(text, link.nextStats(handle))
    }

    @Test
    fun `texts arrive in the order they were sent`() {
        val handle = connect()
        val texts = List(5) { "message $it" }

        texts.forEach { assertTrue(client.send(it)) }

        assertEquals(texts, List(texts.size) { link.nextStats(handle) })
    }

    @Test
    fun `a text sent while the connection is still opening is handed to the link`() {
        client.start()

        assertTrue(client.send("early"))

        assertEquals("early", link.nextStats(link.accept()))
    }

    @Test
    fun `a text sent after a requested reconnect arrives on the new connection`() {
        connect()
        client.reconnect()
        val fresh = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        assertTrue(client.send("again"))

        assertEquals("again", link.nextStats(fresh))
    }
}
