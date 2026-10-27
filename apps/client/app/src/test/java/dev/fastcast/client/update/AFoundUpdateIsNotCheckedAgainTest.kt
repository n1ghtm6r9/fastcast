package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.File
import java.io.IOException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AFoundUpdateIsNotCheckedAgainTest {

    private val listener = RecordingUpdateListener()
    private val fetches = AtomicInteger()
    private val downloadMayEnd = CountDownLatch(1)
    private val installed = CountDownLatch(1)
    private var installs = false
    private val runner = anUpdateRunner(
        listener,
        cache = UpdateCheckCache { 0L },
        fetch = {
            fetches.incrementAndGet()
            anUpdateManifest(versionCode = 101)
        },
        download = { _, _, _ ->
            downloadMayEnd.await()
            if (!installs) throw IOException("stop here")
            File.createTempFile("fastcast-update", ".apk")
        },
        install = { _, _ -> installed.countDown() },
    )

    @After
    fun tearDown() {
        downloadMayEnd.countDown()
        runner.stop()
    }

    @Test
    fun `an update under way is not checked again`() {
        runner.start()
        assertEquals("found 0.1.101", listener.nextEvent())

        runner.checkAgain()
        downloadMayEnd.countDown()

        assertEquals("failed stop here", listener.nextEvent())
        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `an update that failed is left for the next start`() {
        downloadMayEnd.countDown()
        runner.start()
        assertEquals(listOf("found 0.1.101", "failed stop here"), List(2) { listener.nextEvent() })

        runner.checkAgain()

        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(1, fetches.get())
    }

    @Test
    fun `an installed update is not checked again`() {
        installs = true
        downloadMayEnd.countDown()
        runner.start()
        assertEquals("found 0.1.101", listener.nextEvent())
        assertTrue(installed.await(5L, TimeUnit.SECONDS))
        assertTrue(listener.hasNoEventWithin(200L))

        runner.checkAgain()

        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(1, fetches.get())
    }
}
