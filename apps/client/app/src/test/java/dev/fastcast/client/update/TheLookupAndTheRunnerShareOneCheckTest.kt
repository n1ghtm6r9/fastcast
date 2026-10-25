package dev.fastcast.client.update

import dev.fastcast.client.discovery.DirectExecutor
import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.IOException
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheLookupAndTheRunnerShareOneCheckTest {

    private var nowMs = 0L
    private val cache = UpdateCheckCache { nowMs }
    private val fetches = AtomicInteger()
    private var found: UpdateManifest? = null
    private val listener = RecordingUpdateListener()
    private val callbacks = LinkedBlockingQueue<String>()
    private val runner = anUpdateRunner(
        listener,
        cache = cache,
        fetch = ::countedFetch,
        download = { _, _, isCancelled ->
            while (!isCancelled()) Thread.sleep(POLL_MS)
            throw IOException("cancelled")
        },
    )
    private val lookup = UpdateLookup(
        cache = cache,
        fetch = ::countedFetch,
        deliver = DirectExecutor,
        onUpdateAvailable = { callbacks.add(CALLBACK) },
    )

    @After
    fun tearDown() {
        runner.stop()
        lookup.stop()
    }

    @Test
    fun `a lookup after a check that found nothing sends no second request`() {
        runner.start()
        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())

        lookup.start()

        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a lookup after a check that found an update calls back without a second request`() {
        found = anUpdateManifest(versionCode = 101)
        runner.start()
        assertEquals("found 0.1.101", listener.nextEvent())

        lookup.start()

        assertEquals(CALLBACK, callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a check after a lookup that found an update announces it without a second request`() {
        found = anUpdateManifest(versionCode = 101)
        lookup.start()
        assertEquals(CALLBACK, callbacks.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertEquals(1, fetches.get())
    }

    @Test
    fun `a check after a lookup that found nothing sends no second request`() {
        lookup.start()
        assertNull(callbacks.poll(300L, TimeUnit.MILLISECONDS))

        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        assertEquals(1, fetches.get())
    }

    @Test
    fun `both ask again once the shared result has expired`() {
        runner.start()
        listener.nextEvent()
        nowMs += UpdateCheckCache.REUSE_MS

        lookup.start()
        callbacks.poll(300L, TimeUnit.MILLISECONDS)

        assertEquals(2, fetches.get())
    }

    private fun countedFetch(): UpdateManifest? {
        fetches.incrementAndGet()
        return found
    }

    private companion object {
        const val CALLBACK = "update available"
        const val TIMEOUT_SECONDS = 5L
        const val POLL_MS = 5L
    }
}
