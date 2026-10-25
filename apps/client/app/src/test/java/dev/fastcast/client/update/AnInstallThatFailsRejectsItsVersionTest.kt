package dev.fastcast.client.update

import dev.fastcast.client.testing.aRejectedVersions
import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import dev.fastcast.client.testing.awaitTrue
import java.io.File
import java.io.IOException
import java.util.concurrent.CountDownLatch
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class AnInstallThatFailsRejectsItsVersionTest {

    private val listener = RecordingUpdateListener()
    private val manifest = anUpdateManifest(versionCode = 101)
    private val rejected = aRejectedVersions()
    private val runners = mutableListOf<UpdateRunner>()

    @After
    fun tearDown() = runners.forEach(UpdateRunner::stop)

    @Test
    fun `a failed install rejects its version`() {
        start(install = { _, _ -> throw IllegalStateException("refused") })
        List(2) { listener.nextEvent() }

        assertTrue(rejected.isRejected(101))
    }

    @Test
    fun `a failed install is not offered to a later run`() {
        start(install = { _, _ -> throw IllegalStateException("refused") })
        List(2) { listener.nextEvent() }
        runners.forEach(UpdateRunner::stop)

        start(cache = UpdateCheckCache { 0L })

        assertEquals(RecordingUpdateListener.NO_UPDATE, listener.nextEvent())
    }

    @Test
    fun `a failed download does not reject its version`() {
        start(download = { _, _, _ -> throw IOException("boom") })
        List(2) { listener.nextEvent() }

        assertFalse(rejected.isRejected(101))
    }

    @Test
    fun `an install that succeeds does not reject its version`() {
        val installed = CountDownLatch(1)

        start(install = { _, _ -> installed.countDown() })

        assertTrue(installed.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))
        listener.nextEvent()
        listener.hasNoEventWithin(200L)
        assertFalse(rejected.isRejected(101))
    }

    @Test
    fun `an install that fails after the run was stopped still rejects its version`() {
        val installing = CountDownLatch(1)
        val release = CountDownLatch(1)
        val runner = start(install = { _, _ ->
            installing.countDown()
            release.await(TIMEOUT_SECONDS, TimeUnit.SECONDS)
            throw IllegalStateException("refused")
        })
        assertTrue(installing.await(TIMEOUT_SECONDS, TimeUnit.SECONDS))

        runner.stop()
        release.countDown()

        assertTrue(awaitTrue { rejected.isRejected(101) })
    }

    private fun start(
        cache: UpdateCheckCache = UpdateCheckCache { 0L },
        download: (UpdateManifest, (Long, Long) -> Unit, () -> Boolean) -> File = { _, _, _ -> File("unused") },
        install: (File, UpdateManifest) -> Unit = { _, _ -> error("no install") },
    ): UpdateRunner =
        anUpdateRunner(
            listener,
            cache = cache,
            rejected = rejected,
            fetch = { manifest },
            download = download,
            install = install,
        ).also { started ->
            runners.add(started)
            started.start()
        }

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
