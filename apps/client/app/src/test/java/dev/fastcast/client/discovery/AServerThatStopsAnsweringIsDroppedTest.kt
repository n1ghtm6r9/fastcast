package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AServerThatStopsAnsweringIsDroppedTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener, staleAfterMs = 200L)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `a server that no longer answers leaves the list`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        assertEquals(listOf("Gaming PC"), listener.nextReport().map { it.name })

        responder.answerWith()

        assertEquals(emptyList<DiscoveredServer>(), listener.nextReport())
    }

    @Test
    fun `only the server that went quiet leaves the list`() {
        responder.answerWith(aServerReply("Gaming PC", 3278), aServerReply("Laptop", 4000))
        finder.start()
        listener.nextReport()
        assertEquals(listOf("Gaming PC", "Laptop"), listener.nextReport().map { it.name })

        responder.answerWith(aServerReply("Laptop", 4000))

        assertEquals(listOf("Laptop"), listener.nextReport().map { it.name })
    }

    @Test
    fun `a server that keeps answering stays listed`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        listener.nextReport()

        assertTrue(listener.hasNoReportWithin(600L))
    }
}
