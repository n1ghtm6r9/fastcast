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

class TheDownloaderFollowsRedirectsTest {

    @get:Rule
    val folder = TemporaryFolder()

    private val server = FakeUpdateServer()
    private val payload = anApkPayload(80_000)
    private val manifest = anUpdateManifestFor(payload, server.urlOf(RELEASE_PATH))
    private val downloader by lazy { UpdateDownloader(File(folder.root, "updates"), UpdateHttpClient.create()) }

    @After
    fun tearDown() = server.close()

    @Test
    fun `a 302 to the object store delivers the apk`() {
        server.redirect(RELEASE_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, payload)

        val apk = downloader.download(manifest, { _, _ -> })

        assertArrayEquals(payload, apk.readBytes())
    }

    @Test
    fun `each hop of the redirect is requested once`() {
        server.redirect(RELEASE_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, payload)

        downloader.download(manifest, { _, _ -> })

        assertEquals(listOf(1, 1), listOf(server.requestsTo(RELEASE_PATH), server.requestsTo(OBJECT_PATH)))
    }

    @Test
    fun `a chain of redirects delivers the apk`() {
        server.redirect(RELEASE_PATH, SECOND_HOP_PATH, status = 301)
        server.redirect(SECOND_HOP_PATH, OBJECT_PATH, status = 307)
        server.serve(OBJECT_PATH, payload)

        val apk = downloader.download(manifest, { _, _ -> })

        assertArrayEquals(payload, apk.readBytes())
    }

    @Test
    fun `a redirect that switches to another protocol is not followed`() {
        server.redirectToUrl(RELEASE_PATH, "https://127.0.0.1:1$OBJECT_PATH")

        assertThrows(IOException::class.java) { downloader.download(manifest, { _, _ -> }) }
    }

    private companion object {
        const val RELEASE_PATH = "/releases/download/v0.1.101/fastcast-tv.apk"
        const val SECOND_HOP_PATH = "/release-assets/second-hop"
        const val OBJECT_PATH = "/object-store/apk-blob"
    }
}
