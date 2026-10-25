package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import java.net.InetAddress
import java.net.ServerSocket
import java.util.concurrent.TimeUnit
import okhttp3.OkHttpClient
import org.junit.After
import org.junit.Assert.assertNull
import org.junit.Test

class AnUnreachableManifestIsNotAnUpdateTest {

    private val server = FakeUpdateServer()

    @After
    fun tearDown() = server.close()

    @Test
    fun `a refused connection is no update`() {
        val closedPort = ServerSocket(0, 0, InetAddress.getLoopbackAddress()).use { it.localPort }
        val checker = UpdateChecker(UpdateHttpClient.create(), "http://127.0.0.1:$closedPort$MANIFEST_PATH", 100)

        assertNull(checker.check())
    }

    @Test
    fun `a url that cannot be parsed is no update`() {
        assertNull(UpdateChecker(UpdateHttpClient.create(), "not a url", 100).check())
    }

    @Test
    fun `a server that never answers is no update`() {
        server.neverAnswer(MANIFEST_PATH)
        val impatient = OkHttpClient.Builder().readTimeout(300, TimeUnit.MILLISECONDS).build()

        assertNull(UpdateChecker(impatient, server.urlOf(MANIFEST_PATH), 100).check())
    }

    @Test
    fun `an answer cut short is no update`() {
        val json = anUpdateManifest().asJson().toByteArray()
        server.cutShort(MANIFEST_PATH, json.copyOf(json.size / 2), announcedSize = json.size.toLong())

        assertNull(UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), 100).check())
    }

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
