package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import java.io.IOException
import org.junit.After
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class ACancelledDownloadStopsAndLeavesNoFileTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(2_000_000)
    private val manifest = anUpdateManifestFor(payload, server.urlOf(APK_PATH))
    private val directory by lazy { File(folder.root, "updates") }
    private val downloader by lazy { UpdateDownloader(directory, UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `a download cancelled midway fails`() {
        server.serve(APK_PATH, payload)
        var cancelled = false

        assertThrows(IOException::class.java) {
            downloader.download(manifest, { _, _ -> cancelled = true }, { cancelled })
        }
    }

    @Test
    fun `a download cancelled midway leaves no file`() {
        server.serve(APK_PATH, payload)
        var cancelled = false

        runCatching { downloader.download(manifest, { _, _ -> cancelled = true }, { cancelled }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `the cancel is noticed before another chunk is read`() {
        server.serve(APK_PATH, payload)
        var cancelled = false
        var reports = 0

        runCatching {
            downloader.download(manifest, { _, _ ->
                reports++
                cancelled = true
            }, { cancelled })
        }

        assertEquals(1, reports)
    }

    @Test
    fun `a download cancelled before it starts reports nothing and leaves no file`() {
        server.serve(APK_PATH, payload)
        var reports = 0

        assertThrows(IOException::class.java) {
            downloader.download(manifest, { _, _ -> reports++ }, { true })
        }

        assertEquals(0, reports)
        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `a download that is never cancelled completes`() {
        server.serve(APK_PATH, payload)

        val apk = downloader.download(manifest, { _, _ -> }, { false })

        assertArrayEquals(payload, apk.readBytes())
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
