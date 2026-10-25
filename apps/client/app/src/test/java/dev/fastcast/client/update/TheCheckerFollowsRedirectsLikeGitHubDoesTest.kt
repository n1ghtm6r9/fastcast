package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheCheckerFollowsRedirectsLikeGitHubDoesTest {

    private val server = FakeUpdateServer()
    private val manifest = anUpdateManifest(versionCode = 102)

    @After
    fun tearDown() = server.close()

    @Test
    fun `a 302 to the object store is followed`() {
        server.redirect(LATEST_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, checkerFor(LATEST_PATH).check())
    }

    @Test
    fun `each hop of the redirect is requested once`() {
        server.redirect(LATEST_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, manifest.asJson().toByteArray())

        checkerFor(LATEST_PATH).check()

        assertEquals(listOf(1, 1), listOf(server.requestsTo(LATEST_PATH), server.requestsTo(OBJECT_PATH)))
    }

    @Test
    fun `every redirect status is followed`() {
        server.serve(OBJECT_PATH, manifest.asJson().toByteArray())

        listOf(301, 302, 303, 307, 308).forEach { status ->
            val path = "/redirect-with-$status"
            server.redirect(path, OBJECT_PATH, status)

            assertEquals("status $status", manifest, checkerFor(path).check())
        }
    }

    @Test
    fun `a chain of redirects is followed`() {
        server.redirect(LATEST_PATH, SECOND_HOP_PATH)
        server.redirect(SECOND_HOP_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, checkerFor(LATEST_PATH).check())
    }

    @Test
    fun `a redirect loop is given up on`() {
        server.redirect(LATEST_PATH, SECOND_HOP_PATH)
        server.redirect(SECOND_HOP_PATH, LATEST_PATH)

        assertNull(checkerFor(LATEST_PATH).check())
    }

    @Test
    fun `a redirect that switches to another protocol is not followed`() {
        server.redirectToUrl(LATEST_PATH, "https://127.0.0.1:1$OBJECT_PATH")

        assertNull(checkerFor(LATEST_PATH).check())
    }

    @Test
    fun `a redirect to a missing file is no update`() {
        server.redirect(LATEST_PATH, OBJECT_PATH)
        server.serve(OBJECT_PATH, "Not Found".toByteArray(), status = 404)

        assertNull(checkerFor(LATEST_PATH).check())
    }

    private fun checkerFor(path: String) = UpdateChecker(UpdateHttpClient.create(), server.urlOf(path), 100)

    private companion object {
        const val LATEST_PATH = "/latest/download/fastcast-tv.json"
        const val SECOND_HOP_PATH = "/release-assets/second-hop"
        const val OBJECT_PATH = "/object-store/manifest-blob"
    }
}
