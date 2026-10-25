package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import dev.fastcast.client.testing.awaitTrue
import java.io.File
import java.io.IOException
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class AFailedUpdateIsReportedAndForgottenTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val listener = RecordingUpdateListener()
    private val cache = UpdateCheckCache { 0L }
    private val manifest = anUpdateManifest(versionCode = 101)
    private val installs = AtomicInteger()
    private var runner: UpdateRunner? = null

    @After
    fun tearDown() {
        runner?.stop()
    }

    @Test
    fun `a failed download is reported with its cause`() {
        start(download = { _, _, _ -> throw IOException("boom") })

        assertEquals(listOf("found 0.1.101", "failed boom"), List(2) { listener.nextEvent() })
    }

    @Test
    fun `a failed download never reaches the installer`() {
        start(download = { _, _, _ -> throw IOException("boom") })
        List(2) { listener.nextEvent() }

        assertTrue(listener.hasNoEventWithin(200L))
        assertEquals(0, installs.get())
    }

    @Test
    fun `a failed install is reported with its cause`() {
        start(
            download = { _, _, _ -> folder.newFile() },
            install = { _, _ -> throw IllegalStateException("refused") },
        )

        assertEquals(listOf("found 0.1.101", "failed refused"), List(2) { listener.nextEvent() })
    }

    @Test
    fun `a file left by a failed install is deleted`() {
        val apk = folder.newFile("fastcast-tv-101.apk")
        start(
            download = { _, _, _ -> apk },
            install = { _, _ -> throw IllegalStateException("refused") },
        )
        List(2) { listener.nextEvent() }

        assertTrue(awaitTrue { !apk.exists() })
    }

    @Test
    fun `a failing check is reported with its cause`() {
        start(fetch = { throw IOException("offline") })

        assertEquals("failed offline", listener.nextEvent())
    }

    @Test
    fun `a failed download is forgotten by the shared cache`() {
        start(download = { _, _, _ -> throw IOException("boom") })
        List(2) { listener.nextEvent() }

        assertNull(cache.resolve { error("must not fetch") })
    }

    @Test
    fun `a failed install is forgotten by the shared cache`() {
        start(
            download = { _, _, _ -> folder.newFile() },
            install = { _, _ -> throw IllegalStateException("refused") },
        )
        List(2) { listener.nextEvent() }

        assertNull(cache.resolve { error("must not fetch") })
    }

    @Test
    fun `a failing check is forgotten by the shared cache`() {
        start(fetch = { throw IOException("offline") })
        listener.nextEvent()

        assertNull(cache.resolve { error("must not fetch") })
    }

    private fun start(
        fetch: () -> UpdateManifest? = { manifest },
        download: (UpdateManifest, (Long, Long) -> Unit, () -> Boolean) -> File = { _, _, _ -> error("no download") },
        install: (File, UpdateManifest) -> Unit = { _, _ -> installs.incrementAndGet() },
    ) {
        runner = anUpdateRunner(listener, cache = cache, fetch = fetch, download = download, install = install)
            .also(UpdateRunner::start)
    }
}
