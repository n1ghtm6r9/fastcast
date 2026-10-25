package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import org.junit.After
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class ADownloadedApkIsSavedUnderItsVersionCodeTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(200_000)
    private val directory by lazy { File(folder.root, "updates") }
    private val downloader by lazy { UpdateDownloader(directory, UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `the apk is saved as fastcast-tv-101 in the updates directory`() {
        server.serve(APK_PATH, payload)

        val apk = downloader.download(manifest(101), { _, _ -> })

        assertEquals(File(directory, "fastcast-tv-101.apk"), apk)
    }

    @Test
    fun `the file name follows the version code of the manifest`() {
        server.serve(APK_PATH, payload)

        val apk = downloader.download(manifest(207), { _, _ -> })

        assertEquals("fastcast-tv-207.apk", apk.name)
    }

    @Test
    fun `the saved bytes equal the served bytes`() {
        server.serve(APK_PATH, payload)

        val apk = downloader.download(manifest(101), { _, _ -> })

        assertArrayEquals(payload, apk.readBytes())
    }

    @Test
    fun `the directory holds nothing but the apk`() {
        server.serve(APK_PATH, payload)

        val apk = downloader.download(manifest(101), { _, _ -> })

        assertEquals(listOf(apk), directory.listFiles()?.toList())
    }

    @Test
    fun `the updates directory is created when it is missing`() {
        server.serve(APK_PATH, payload)
        assertFalse(directory.exists())

        downloader.download(manifest(101), { _, _ -> })

        assertTrue(directory.isDirectory)
    }

    @Test
    fun `an upper case digest in the manifest still matches`() {
        server.serve(APK_PATH, payload)
        val shouting = manifest(101).let { it.copy(sha256 = it.sha256.uppercase()) }

        val apk = downloader.download(shouting, { _, _ -> })

        assertArrayEquals(payload, apk.readBytes())
    }

    private fun manifest(versionCode: Int) = anUpdateManifestFor(payload, server.urlOf(APK_PATH), versionCode)

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
