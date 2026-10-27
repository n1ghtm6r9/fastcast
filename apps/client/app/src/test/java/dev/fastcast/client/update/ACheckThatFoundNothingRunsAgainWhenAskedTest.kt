package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.IOException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ACheckThatFoundNothingRunsAgainWhenAskedTest {

    private val listener = RecordingUpdateListener()
    private var nowMs = 0L
    private val cache = UpdateCheckCache { nowMs }
    private val fetches = AtomicInteger()
    private var found: UpdateManifest? = null
    private var offline = false
    private var fetchGate: CountDownLatch? = null
    private val runner = anUpdateRunner(
        listener,
        cache = cache,
        fetch = {
            fetches.incrementAndGet()
            fetchGate?.await()
            if (offline) throw IOException("offline")
            found
        },
        download = { _, _, _ -> throw IOException("stop here") },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `a check that found nothing asks again once the reuse window is over`() {
        runner.start()
        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        nowMs += UpdateCheckCache.REUSE_MS
        found = anUpdateManifest(versionCode = 101)

        runner.checkAgain()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertEquals(2, fetches.get())
    }

    @Test
    fun `a check that found nothing is answered from the cache within the window`() {
        runner.start()
        listener.nextEvent()

        runner.checkAgain()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a failed check runs again`() {
        offline = true
        runner.start()
        assertEquals("failed offline", listener.nextEvent())
        offline = false

        runner.checkAgain()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertEquals(2, fetches.get())
    }

    @Test
    fun `a check still under way is not doubled`() {
        val gate = CountDownLatch(1)
        fetchGate = gate
        runner.start()

        runner.checkAgain()
        gate.countDown()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `nothing is checked again before the runner starts`() {
        runner.checkAgain()

        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(0, fetches.get())
    }
}
