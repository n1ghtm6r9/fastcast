package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class SeveralServersAreListedInTheOrderTheyAnsweredTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `each new server is appended to the list`() {
        responder.answerWith(aServerReply("First", 3278), aServerReply("Second", 4000))

        finder.start()

        assertEquals(listOf("First"), listener.nextReport().map { it.name })
        assertEquals(listOf("First", "Second"), listener.nextReport().map { it.name })
    }

    @Test
    fun `the report is a snapshot that later answers do not change`() {
        responder.answerWith(aServerReply("First", 3278), aServerReply("Second", 4000))

        finder.start()
        val firstReport = listener.nextReport()
        listener.nextReport()

        assertEquals(listOf("First"), firstReport.map { it.name })
    }
}
