package dev.fastcast.client.stream

import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheStreamClientSendsTextToTheServerTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient({ server.url }, listener, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    private fun connect(): FakeConnection {
        client.start()
        val connection = server.accept()
        listener.skipTo(RecordingStreamListener.CONNECTED)
        return connection
    }

    @Test
    fun `a short text reaches the server unchanged`() {
        val connection = connect()

        assertTrue(client.send("""{"type":"stats"}"""))

        assertEquals("""{"type":"stats"}""", connection.nextText())
    }

    @Test
    fun `a text longer than one hundred twenty five bytes reaches the server unchanged`() {
        val connection = connect()
        val text = "stats ".repeat(50)

        assertTrue(client.send(text))

        assertEquals(text, connection.nextText())
    }

    @Test
    fun `text outside ascii reaches the server unchanged`() {
        val connection = connect()
        val text = "декодер c2.mtk.hevc.decoder ☃"

        assertTrue(client.send(text))

        assertEquals(text, connection.nextText())
    }

    @Test
    fun `texts arrive in the order they were sent`() {
        val connection = connect()
        val texts = List(5) { "message $it" }

        texts.forEach { assertTrue(client.send(it)) }

        assertEquals(texts, List(texts.size) { connection.nextText() })
    }

    @Test
    fun `a text sent while the connection is still opening arrives once it is open`() {
        client.start()

        assertTrue(client.send("early"))

        assertEquals("early", server.accept().nextText())
    }

    @Test
    fun `a text sent after a requested reconnect arrives on the new connection`() {
        connect()
        client.reconnect()
        val fresh = server.accept()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        assertTrue(client.send("again"))

        assertEquals("again", fresh.nextText())
    }
}
