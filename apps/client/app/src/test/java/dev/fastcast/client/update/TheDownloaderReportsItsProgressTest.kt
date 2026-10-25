package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class TheDownloaderReportsItsProgressTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(200_000)
    private val manifest = anUpdateManifestFor(payload, server.urlOf(APK_PATH))
    private val reports = mutableListOf<Pair<Long, Long>>()
    private val downloader by lazy { UpdateDownloader(File(folder.root, "updates"), UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `progress ends at the full size`() {
        downloadAndRecord()

        assertEquals(payload.size.toLong() to payload.size.toLong(), reports.last())
    }

    @Test
    fun `a large download reports several times`() {
        downloadAndRecord()

        assertTrue("reports: ${reports.size}", reports.size > 1)
    }

    @Test
    fun `progress only moves forward`() {
        downloadAndRecord()

        assertEquals(reports.map { it.first }.sorted().distinct(), reports.map { it.first })
    }

    @Test
    fun `the total is always the announced size`() {
        downloadAndRecord()

        assertEquals(setOf(payload.size.toLong()), reports.map { it.second }.toSet())
    }

    private fun downloadAndRecord() {
        server.serve(APK_PATH, payload)
        downloader.download(manifest, { received, total -> reports += received to total })
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
