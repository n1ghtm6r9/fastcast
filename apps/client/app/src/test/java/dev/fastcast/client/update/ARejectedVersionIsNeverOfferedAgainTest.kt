package dev.fastcast.client.update

import dev.fastcast.client.testing.aRejectedVersions
import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.File
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class ARejectedVersionIsNeverOfferedAgainTest {

    private val listener = RecordingUpdateListener()
    private val cache = UpdateCheckCache { 0L }
    private val rejected = aRejectedVersions()
    private val downloads = AtomicInteger()
    private val installs = AtomicInteger()
    private var published = anUpdateManifest(versionCode = 101)
    private val runner = anUpdateRunner(
        listener,
        cache = cache,
        rejected = rejected,
        fetch = { published },
        download = { _, _, _ ->
            downloads.incrementAndGet()
            File("unused")
        },
        install = { _, _ -> installs.incrementAndGet() },
    )

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `a rejected version is reported as no update`() {
        rejected.reject(101)

        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
    }

    @Test
    fun `a rejected version is neither downloaded nor installed`() {
        rejected.reject(101)

        runner.start()
        listener.nextEvent()
        listener.hasNoEventWithin(200L)

        assertEquals(listOf(0, 0), listOf(downloads.get(), installs.get()))
    }

    @Test
    fun `a version rejected after the check was cached is not offered either`() {
        cache.resolve { published }
        rejected.reject(101)

        runner.start()

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
        listener.hasNoEventWithin(200L)
        assertEquals(0, downloads.get())
    }

    @Test
    fun `a newer version than the rejected one is announced`() {
        rejected.reject(101)
        published = anUpdateManifest(versionCode = 102)

        runner.start()

        assertEquals("found 0.1.102", listener.nextEvent())
    }

    @Test
    fun `a newer version than the rejected one is downloaded and installed`() {
        rejected.reject(101)
        published = anUpdateManifest(versionCode = 102)

        runner.start()
        listener.nextEvent()
        listener.hasNoEventWithin(200L)

        assertEquals(listOf(1, 1), listOf(downloads.get(), installs.get()))
    }
}
