package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.IOException
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AFailedCheckIsRetriedByTheNextRunTest {

    private val listener = RecordingUpdateListener()
    private val cache = UpdateCheckCache { 0L }
    private val manifest = anUpdateManifest(versionCode = 101)
    private val fetches = AtomicInteger()
    private var offline = true
    private val runner = anUpdateRunner(
        listener,
        cache = cache,
        fetch = {
            fetches.incrementAndGet()
            if (offline) throw IOException("offline")
            manifest
        },
        download = { _, _, _ -> throw IOException("stop here") },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `a restarted runner checks again after a failed check`() {
        runner.start()
        assertEquals("failed offline", listener.nextEvent())
        runner.stop()
        offline = false

        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
        assertEquals(2, fetches.get())
    }

    @Test
    fun `a failed check leaves no result in the shared cache`() {
        runner.start()
        listener.nextEvent()

        assertEquals(manifest, cache.resolve { manifest })
    }
}
