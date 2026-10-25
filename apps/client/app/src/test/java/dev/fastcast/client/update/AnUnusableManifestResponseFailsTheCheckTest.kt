package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateManifestJson
import dev.fastcast.client.testing.asJson
import java.io.IOException
import org.junit.After
import org.junit.Assert.assertThrows
import org.junit.Test

class AnUnusableManifestResponseFailsTheCheckTest {

    private val server = FakeUpdateServer()
    private val checker = UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), 100)

    @After
    fun tearDown() = server.close()

    @Test
    fun `a release that does not exist yet fails the check`() {
        server.serve(MANIFEST_PATH, "Not Found".toByteArray(), status = 404)

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `a server error fails the check`() {
        server.serve(MANIFEST_PATH, "Oops".toByteArray(), status = 500)

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `an error status fails the check even with a valid manifest in the body`() {
        server.serve(MANIFEST_PATH, anUpdateManifest().asJson().toByteArray(), status = 404)

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `an empty answer fails the check`() {
        server.serve(MANIFEST_PATH, ByteArray(0))

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `an answer without content fails the check`() {
        server.serve(MANIFEST_PATH, ByteArray(0), status = 204)

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `an html page fails the check`() {
        server.serve(MANIFEST_PATH, "<html><body>Sign in</body></html>".toByteArray())

        assertThrows(IOException::class.java) { checker.check() }
    }

    @Test
    fun `a manifest the parser rejects fails the check`() {
        server.serve(MANIFEST_PATH, anUpdateManifestJson("url" to "http://github.com/fastcast-tv.apk").toByteArray())

        assertThrows(IOException::class.java) { checker.check() }
    }

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
