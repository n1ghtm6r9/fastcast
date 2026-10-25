package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import java.io.IOException
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class ADownloadWithTheWrongDigestIsRejectedTest {

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
    fun `an unrelated digest of the right size fails`() {
        server.serve(APK_PATH, payload)

        assertThrows(IOException::class.java) {
            downloader.download(honest.copy(sha256 = "0".repeat(64)), { _, _ -> })
        }
    }

    @Test
    fun `an unrelated digest leaves no file`() {
        server.serve(APK_PATH, payload)

        runCatching { downloader.download(honest.copy(sha256 = "0".repeat(64)), { _, _ -> }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `a payload that differs by one bit fails and leaves no file`() {
        val flipped = payload.copyOf().also { it[1_000] = (it[1_000].toInt() xor 1).toByte() }
        server.serve(APK_PATH, flipped)

        assertThrows(IOException::class.java) { downloader.download(honest, { _, _ -> }) }

        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    @Test
    fun `a payload of the same size with other content fails`() {
        server.serve(APK_PATH, ByteArray(payload.size))

        assertThrows(IOException::class.java) { downloader.download(honest, { _, _ -> }) }
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
