package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateManifestJson
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertNull
import org.junit.Test

class AnUnusableManifestResponseIsNotAnUpdateTest {

    private val server = FakeUpdateServer()
    private val checker = UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), 100)

    @After
    fun tearDown() = server.close()

    @Test
    fun `a release that does not exist yet is no update`() {
        server.serve(MANIFEST_PATH, "Not Found".toByteArray(), status = 404)

        assertNull(checker.check())
    }

    @Test
    fun `a server error is no update`() {
        server.serve(MANIFEST_PATH, "Oops".toByteArray(), status = 500)

        assertNull(checker.check())
    }

    @Test
    fun `an error status is no update even with a valid manifest in the body`() {
        server.serve(MANIFEST_PATH, anUpdateManifest().asJson().toByteArray(), status = 404)

        assertNull(checker.check())
    }

    @Test
    fun `an empty answer is no update`() {
        server.serve(MANIFEST_PATH, ByteArray(0))

        assertNull(checker.check())
    }

    @Test
    fun `an answer without content is no update`() {
        server.serve(MANIFEST_PATH, ByteArray(0), status = 204)

        assertNull(checker.check())
    }

    @Test
    fun `an html page is no update`() {
        server.serve(MANIFEST_PATH, "<html><body>Sign in</body></html>".toByteArray())

        assertNull(checker.check())
    }

    @Test
    fun `a manifest the parser rejects is no update`() {
        server.serve(MANIFEST_PATH, anUpdateManifestJson("url" to "http://github.com/fastcast-tv.apk").toByteArray())

        assertNull(checker.check())
    }

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
