package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class ARenamedServerReplacesItsOldNameTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `the newest name of the same host and port wins`() {
        responder.answerWith(aServerReply("Old name", 3278))
        finder.start()
        assertEquals(listOf("Old name"), listener.nextReport().map { it.name })

        responder.answerWith(aServerReply("New name", 3278))

        assertEquals(listOf("New name"), listener.nextReport().map { it.name })
    }

    @Test
    fun `the same host on another port is a second server`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        listener.nextReport()

        responder.answerWith(aServerReply("Gaming PC", 3278), aServerReply("Gaming PC", 4000))

        assertEquals(listOf(3278, 4000), listener.nextReport().map { it.port })
    }
}
