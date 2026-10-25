package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Test

class AStoppedFinderSendsNothingMoreTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val finder = aFinderFor(responder, listener)

    @After
    fun tearDown() {
        finder.stop()
        responder.close()
    }

    @Test
    fun `no probe arrives after the finder was stopped`() {
        finder.start()
        responder.nextProbe()

        finder.stop()
        responder.discardProbesUntilQuiet(150L)

        assertTrue(responder.hasNoProbeWithin(300L))
    }

    @Test
    fun `no report arrives after the finder was stopped`() {
        responder.answerWith(aServerReply("Gaming PC", 3278))
        finder.start()
        listener.nextReport()

        finder.stop()
        responder.answerWith(aServerReply("Gaming PC", 3278), aServerReply("Other PC", 4000))

        assertTrue(listener.hasNoReportWithin(400L))
    }

    @Test
    fun `stopping twice or before the start is harmless`() {
        finder.stop()
        finder.start()
        responder.nextProbe()

        finder.stop()
        finder.stop()
    }

    @Test
    fun `starting a running finder again does not double the probes`() {
        val slowFinder = aFinderFor(responder, listener, probeIntervalMs = 5_000L)

        try {
            slowFinder.start()
            slowFinder.start()
            responder.nextProbe()

            assertTrue(responder.hasNoProbeWithin(300L))
        } finally {
            slowFinder.stop()
        }
    }
}
