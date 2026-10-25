package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateRunner
import dev.fastcast.client.testing.awaitTrue
import java.io.File
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class ANewerVersionIsDownloadedAndInstalledTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val listener = RecordingUpdateListener()
    private val manifest = anUpdateManifest(versionCode = 101)
    private val downloaded = LinkedBlockingQueue<UpdateManifest>()
    private val installed = LinkedBlockingQueue<Pair<File, UpdateManifest>>()
    private val apk by lazy { folder.newFile("fastcast-tv-101.apk") }
    private val runner by lazy {
        anUpdateRunner(
            listener,
            fetch = { manifest },
            download = { given, onProgress, _ ->
                downloaded.add(given)
                onProgress(50L, 100L)
                onProgress(100L, 100L)
                apk
            },
            install = { file, given -> installed.add(file to given) },
        )
    }

    @After
    fun tearDown() = runner.stop()

    @Test
    fun `the newer version is announced before anything else`() {
        runner.start()

        assertEquals("found 0.1.101", listener.nextEvent())
    }

    @Test
    fun `the download progress is reported after the announcement`() {
        runner.start()

        assertEquals(
            listOf("found 0.1.101", "progress 50", "progress 100"),
            List(3) { listener.nextEvent() },
        )
    }

    @Test
    fun `the download is asked for the announced manifest`() {
        runner.start()

        assertEquals(manifest, downloaded.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
    }

    @Test
    fun `the downloaded file is handed to the installer with its manifest`() {
        runner.start()

        assertEquals(apk to manifest, installed.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS))
    }

    @Test
    fun `the installer is handed the file only once`() {
        runner.start()
        installed.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS)

        assertNull(installed.poll(200L, TimeUnit.MILLISECONDS))
    }

    @Test
    fun `the downloaded file is deleted once handed over`() {
        runner.start()
        installed.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS)

        assertTrue(awaitTrue { !apk.exists() })
    }

    private companion object {
        const val TIMEOUT_SECONDS = 5L
    }
}
