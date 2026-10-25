package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import java.io.File
import java.io.IOException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicBoolean
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AStoppedRunnerStaysSilentTest {

    private val listener = RecordingUpdateListener()
    private val cache = UpdateCheckCache { 0L }
    private val manifest = anUpdateManifest(versionCode = 101)
    private val downloads = AtomicInteger()
    private val installs = AtomicInteger()
    private var runner: UpdateRunner? = null

    @After
    fun tearDown() {
        runner?.stop()
    }

    @Test
    fun `a run stopped during the check reports nothing`() {
        val checking = CountDownLatch(1)
        val release = CountDownLatch(1)
        val runner = start(fetch = {
            checking.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            manifest
        })
        assertTrue(checking.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()
        release.countDown()

        assertTrue(listener.hasNoEventWithin(300L))
    }

    @Test
    fun `a run stopped during the check downloads nothing`() {
        val checking = CountDownLatch(1)
        val release = CountDownLatch(1)
        val runner = start(fetch = {
            checking.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            manifest
        })
        assertTrue(checking.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()
        release.countDown()
        listener.hasNoEventWithin(300L)

        assertEquals(0, downloads.get())
    }

    @Test
    fun `a run stopped during the download reports nothing more and installs nothing`() {
        val downloading = CountDownLatch(1)
        val finished = CountDownLatch(1)
        val runner = start(download = { _, _, isCancelled ->
            downloading.countDown()
            waitUntil(isCancelled)
            finished.countDown()
            throw IOException("Download cancelled")
        })
        assertEquals("found 0.1.101", listener.nextEvent())
        assertTrue(downloading.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()

        assertTrue(finished.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertTrue(listener.hasNoEventWithin(300L))
        assertEquals(0, installs.get())
    }

    @Test
    fun `a run stopped during the download is told to cancel it`() {
        val downloading = CountDownLatch(1)
        val finished = CountDownLatch(1)
        val toldToCancel = AtomicBoolean(false)
        val runner = start(download = { _, _, isCancelled ->
            downloading.countDown()
            toldToCancel.set(waitUntil(isCancelled))
            finished.countDown()
            throw IOException("Download cancelled")
        })
        assertTrue(downloading.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()

        assertTrue(finished.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertTrue(toldToCancel.get())
    }

    @Test
    fun `a run that is not stopped is never told to cancel`() {
        val told = AtomicBoolean(true)
        val finished = CountDownLatch(1)
        start(download = { _, _, isCancelled ->
            told.set(isCancelled())
            finished.countDown()
            File("unused")
        })

        assertTrue(finished.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        assertEquals(false, told.get())
    }

    @Test
    fun `a stopped run does not make the shared cache forget the update`() {
        val downloading = CountDownLatch(1)
        val finished = CountDownLatch(1)
        val runner = start(download = { _, _, isCancelled ->
            downloading.countDown()
            waitUntil(isCancelled)
            finished.countDown()
            throw IOException("Download cancelled")
        })
        assertTrue(downloading.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()
        assertTrue(finished.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        listener.hasNoEventWithin(300L)

        assertEquals(manifest, cache.resolve { error("must not fetch") })
    }

    private fun start(
        fetch: () -> UpdateManifest? = { manifest },
        download: (UpdateManifest, (Long, Long) -> Unit, () -> Boolean) -> File = { _, _, _ ->
            downloads.incrementAndGet()
            File("unused")
        },
    ): UpdateRunner =
        anUpdateRunner(
            listener,
            cache = cache,
            fetch = fetch,
            download = download,
            install = { _, _ -> installs.incrementAndGet() },
        ).also { started ->
            runner = started
            started.start()
        }

    private fun waitUntil(condition: () -> Boolean): Boolean {
        val deadlineNs = System.nanoTime() + TimeUnit.SECONDS.toNanos(TIMEOUT_SECONDS)
        while (System.nanoTime() < deadlineNs) {
            if (condition()) return true
            Thread.sleep(POLL_MS)
        }
        return false
    }

    private companion object {
        const val TIMEOUT_SECONDS = 5L
        const val POLL_MS = 5L
    }
}
