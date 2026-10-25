package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.anUpdateManifestFor
import java.io.File
import java.io.IOException
import java.net.InetAddress
import java.net.ServerSocket
import java.util.concurrent.TimeUnit
import okhttp3.OkHttpClient
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertThrows
import org.junit.Rule
import org.junit.Test
import org.junit.rules.TemporaryFolder

class AFailedDownloadLeavesNoFileTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(80_000)
    private val directory by lazy { File(folder.root, "updates") }

    @After
    fun tearDown() = server.close()

    @Test
    fun `a missing asset fails`() {
        server.serve(APK_PATH, "Not Found".toByteArray(), status = 404)

        assertFailsWithNoFileLeft(downloaderWith(UpdateHttpClient.create()), server.urlOf(APK_PATH))
    }

    @Test
    fun `a server error fails`() {
        server.serve(APK_PATH, "Oops".toByteArray(), status = 500)

        assertFailsWithNoFileLeft(downloaderWith(UpdateHttpClient.create()), server.urlOf(APK_PATH))
    }

    @Test
    fun `a refused connection fails`() {
        val closedPort = ServerSocket(0, 0, InetAddress.getLoopbackAddress()).use { it.localPort }

        assertFailsWithNoFileLeft(downloaderWith(UpdateHttpClient.create()), "http://127.0.0.1:$closedPort$APK_PATH")
    }

    @Test
    fun `a connection cut in the middle of the body fails`() {
        server.cutShort(APK_PATH, payload.copyOf(payload.size / 2), announcedSize = payload.size.toLong())

        assertFailsWithNoFileLeft(downloaderWith(UpdateHttpClient.create()), server.urlOf(APK_PATH))
    }

    @Test
    fun `a server that never answers fails`() {
        server.neverAnswer(APK_PATH)
        val impatient = OkHttpClient.Builder().readTimeout(300, TimeUnit.MILLISECONDS).build()

        assertFailsWithNoFileLeft(downloaderWith(impatient), server.urlOf(APK_PATH))
    }

    private fun downloaderWith(client: OkHttpClient) = UpdateDownloader(directory, client)

    private fun assertFailsWithNoFileLeft(downloader: UpdateDownloader, url: String) {
        assertThrows(IOException::class.java) {
            downloader.download(anUpdateManifestFor(payload, url), { _, _ -> })
        }
        assertEquals(emptyList<File>(), directory.listFiles()?.toList())
    }

    private companion object {
        const val APK_PATH = "/releases/download/fastcast-tv.apk"
    }
}
