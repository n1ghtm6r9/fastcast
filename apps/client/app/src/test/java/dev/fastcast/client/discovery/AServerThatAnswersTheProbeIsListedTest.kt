package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AServerThatAnswersTheProbeIsListedTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `the server is listed with the host its reply came from and the port it advertised`() {
        responder.answerWith(aServerReply("Gaming PC", 4000))

        finder.start()

        assertEquals(listOf(DiscoveredServer("Gaming PC", "127.0.0.1", 4000)), listener.nextReport())
    }

    @Test
    fun `the listed server offers the address the player connects to`() {
        responder.answerWith(aServerReply("Gaming PC", 4000))

        finder.start()

        assertEquals("127.0.0.1:4000", listener.nextReport().single().address)
    }

    @Test
    fun `nothing is reported while no server answers`() {
        finder.start()
        responder.nextProbe()

        assertTrue(listener.hasNoReportWithin(300L))
    }
}
