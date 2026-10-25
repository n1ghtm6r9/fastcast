package dev.fastcast.client.discovery

import java.net.InetAddress
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheFinderProbesEveryTargetAtOnceAndKeepsProbingTest {

    private val responder = FakeDiscoveryResponder()
    private val listener = RecordingServerFinderListener()
    private val started = mutableListOf<ServerFinder>()

    @After
    fun tearDown() {
        started.forEach(ServerFinder::stop)
        responder.close()
    }

    private fun startFinder(probeIntervalMs: Long, targets: () -> List<InetAddress>) {
        started += aFinderFor(responder, listener, probeIntervalMs = probeIntervalMs, targets = targets).also(ServerFinder::start)
    }

    @Test
    fun `the first probe is the discover message`() {
        startFinder(PROBE_INTERVAL_MS) { listOf(responder.address) }

        assertEquals("""{"type":"discover","version":1}""", responder.nextProbe())
    }

    @Test
    fun `every target gets a probe in the first round without waiting`() {
        startFinder(LONG_INTERVAL_MS) { listOf(responder.address, responder.address) }

        responder.nextProbe()
        responder.nextProbe()

        assertTrue(responder.hasNoProbeWithin(300L))
    }

    @Test
    fun `probes keep coming while the finder runs`() {
        startFinder(PROBE_INTERVAL_MS) { listOf(responder.address) }

        repeat(5) { responder.nextProbe() }
    }

    @Test
    fun `the targets are looked up again for every round`() {
        val lookups = AtomicInteger()
        startFinder(PROBE_INTERVAL_MS) {
            lookups.incrementAndGet()
            listOf(responder.address)
        }

        repeat(3) { responder.nextProbe() }

        assertTrue(lookups.get() >= 3)
    }

    private companion object {
        const val PROBE_INTERVAL_MS = 40L
        const val LONG_INTERVAL_MS = 5_000L
    }
}
