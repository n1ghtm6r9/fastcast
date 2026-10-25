package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ARepeatedAnswerIsListedOnceTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `a server that answers every probe is reported only once`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))

        finder.start()

        assertEquals(1, listener.nextReport().size)
        assertTrue(listener.hasNoReportWithin(400L))
    }

    @Test
    fun `the same server answering twice per probe is listed once`() {
        responder.answerWith(aServerReply("Gaming PC", 3278), aServerReply("Gaming PC", 3278))

        finder.start()

        assertEquals(1, listener.nextReport().size)
        assertTrue(listener.hasNoReportWithin(400L))
    }
}
