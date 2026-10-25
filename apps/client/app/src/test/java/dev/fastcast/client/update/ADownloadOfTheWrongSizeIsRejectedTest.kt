package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import java.io.IOException
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class ADownloadOfTheWrongSizeIsRejectedTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(60_000)
    private val honest = anUpdateManifestFor(payload, server.urlOf(APK_PATH))
    private val directory by lazy { File(folder.root, "updates") }
    private val downloader by lazy { UpdateDownloader(directory, UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `fewer bytes than announced fail`() {
        server.serve(APK_PATH, payload)

        assertThrows(IOException::class.java) {
            downloader.download(honest.copy(size = payload.size + 10L), { _, _ -> })
        }
    }

    @Test
    fun `fewer bytes than announced leave no file`() {
        server.serve(APK_PATH, payload)

        runCatching { downloader.download(honest.copy(size = payload.size + 10L), { _, _ -> }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `more bytes than announced fail`() {
        server.serve(APK_PATH, payload)

        assertThrows(IOException::class.java) {
            downloader.download(honest.copy(size = payload.size - 10L), { _, _ -> })
        }
    }

    @Test
    fun `more bytes than announced leave no file`() {
        server.serve(APK_PATH, payload)

        runCatching { downloader.download(honest.copy(size = payload.size - 10L), { _, _ -> }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `one byte too few fails and leaves no file`() {
        server.serve(APK_PATH, payload)

        assertThrows(IOException::class.java) {
            downloader.download(honest.copy(size = payload.size + 1L), { _, _ -> })
        }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `one byte too many fails and leaves no file`() {
        server.serve(APK_PATH, payload)

        assertThrows(IOException::class.java) {
            downloader.download(honest.copy(size = payload.size - 1L), { _, _ -> })
        }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `an oversized body is never reported beyond the announced size`() {
        server.serve(APK_PATH, anApkPayload(2_000_000))
        val reports = mutableListOf<Long>()

        runCatching { downloader.download(honest, { received, _ -> reports += received }) }

        assertTrue(reports.isNotEmpty() && reports.all { it <= payload.size })
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
