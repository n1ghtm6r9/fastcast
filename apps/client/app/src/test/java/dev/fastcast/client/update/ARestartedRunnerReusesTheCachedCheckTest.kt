package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.IOException
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class ARestartedRunnerReusesTheCachedCheckTest {

    private val listener = RecordingUpdateListener()
    private var nowMs = 0L
    private val cache = UpdateCheckCache { nowMs }
    private val fetches = AtomicInteger()
    private var found: UpdateManifest? = null
    private val runner = anUpdateRunner(
        listener,
        cache = cache,
        fetch = {
            fetches.incrementAndGet()
            found
        },
        download = { _, _, _ -> throw IOException("stop here") },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `starting twice runs a single flow`() {
        runner.start()
        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a restarted runner reuses the cached check`() {
        runner.start()
        listener.nextEvent()
        runner.stop()

        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a restarted runner checks again once the cache has expired`() {
        runner.start()
        listener.nextEvent()
        runner.stop()
        nowMs += UpdateCheckCache.REUSE_MS

        runner.start()
        listener.nextEvent()

        assertEquals(2, fetches.get())
    }

    @Test
    fun `a restart after a failed update sees no update within the window`() {
        found = anUpdateManifest(versionCode = 101)
        runner.start()
        assertEquals(listOf("found 0.1.101", "failed stop here"), List(2) { listener.nextEvent() })
        runner.stop()
        found = anUpdateManifest(versionCode = 102)

        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertEquals(1, fetches.get())
    }

    @Test
    fun `an update found earlier within the window is announced without a new request`() {
        found = anUpdateManifest(versionCode = 101)
        cache.resolve { found }

        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertEquals(0, fetches.get())
    }
}
