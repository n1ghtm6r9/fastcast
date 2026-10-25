package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AStoppedFinderCanBeStartedAgainTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `a restarted finder finds the same server again from an empty list`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        val firstReport = listener.nextReport()

        finder.stop()
        finder.start()

        assertEquals(firstReport, listener.nextReport())
    }

    @Test
    fun `a restarted finder does not remember servers from the earlier run`() {
        responder.answerWith(aServerReply("Old PC", 3278))
        finder.start()
        listener.nextReport()
        finder.stop()

        responder.answerWith(aServerReply("New PC", 4000))
        finder.start()

        assertEquals(listOf("New PC"), listener.nextReport().map { it.name })
    }

    @Test
    fun `a restarted finder probes again`() {
        finder.start()
        responder.nextProbe()
        finder.stop()
        responder.discardProbesUntilQuiet(150L)

        finder.start()

        responder.nextProbe()
    }
}
