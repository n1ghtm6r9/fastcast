package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import java.io.IOException
import org.junit.After
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertThrows
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class TheDownloaderClearsOlderFilesFirstTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(40_000)
    private val manifest = anUpdateManifestFor(payload, server.urlOf(APK_PATH), versionCode = 101)
    private val directory by lazy { File(folder.root, "updates").apply { mkdirs() } }
    private val downloader by lazy { UpdateDownloader(directory, UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `an apk of an earlier version is removed`() {
        server.serve(APK_PATH, payload)
        val earlier = File(directory, "fastcast-tv-99.apk").apply { writeBytes(byteArrayOf(1, 2, 3)) }

        downloader.download(manifest, { _, _ -> })

        assertFalse(earlier.exists())
    }

    @Test
    fun `any other leftover is removed too`() {
        server.serve(APK_PATH, payload)
        val leftover = File(directory, "partial.tmp").apply { writeBytes(byteArrayOf(1)) }

        downloader.download(manifest, { _, _ -> })

        assertFalse(leftover.exists())
    }

    @Test
    fun `a leftover with the target name is replaced`() {
        server.serve(APK_PATH, payload)
        File(directory, "fastcast-tv-101.apk").writeBytes(byteArrayOf(9, 9, 9))

        val apk = downloader.download(manifest, { _, _ -> })

        assertArrayEquals(payload, apk.readBytes())
    }

    @Test
    fun `leftovers are removed even when the download then fails`() {
        server.serve(APK_PATH, "Not Found".toByteArray(), status = 404)
        val earlier = File(directory, "fastcast-tv-99.apk").apply { writeBytes(byteArrayOf(1, 2, 3)) }

        assertThrows(IOException::class.java) { downloader.download(manifest, { _, _ -> }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
        assertFalse(earlier.exists())
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
