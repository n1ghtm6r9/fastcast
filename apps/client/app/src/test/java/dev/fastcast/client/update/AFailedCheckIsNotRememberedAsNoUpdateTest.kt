package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AFailedCheckIsNotRememberedAsNoUpdateTest {

    private val server = FakeUpdateServer()
    private val cache = UpdateCheckCache { 0L }
    private val checker = UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), 100)
    private val manifest = anUpdateManifest(versionCode = 101)

    @After
    fun tearDown() = server.close()

    @Test
    fun `an update published after a server error is found by the next request`() {
        server.serve(MANIFEST_PATH, "Oops".toByteArray(), status = 500)
        runCatching { cache.resolve(checker::check) }
        server.serve(MANIFEST_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, cache.resolve(checker::check))
    }

    @Test
    fun `an update published after an unusable answer is found by the next request`() {
        server.serve(MANIFEST_PATH, "<html><body>Sign in</body></html>".toByteArray())
        runCatching { cache.resolve(checker::check) }
        server.serve(MANIFEST_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, cache.resolve(checker::check))
    }

    @Test
    fun `an update published after an answer cut short is found by the next request`() {
        val json = manifest.asJson().toByteArray()
        server.cutShort(MANIFEST_PATH, json.copyOf(json.size / 2), announcedSize = json.size.toLong())
        runCatching { cache.resolve(checker::check) }
        server.serve(MANIFEST_PATH, json)

        assertEquals(manifest, cache.resolve(checker::check))
    }

    @Test
    fun `a failed check asks the server again on every request`() {
        server.serve(MANIFEST_PATH, "Oops".toByteArray(), status = 500)

        repeat(3) { runCatching { cache.resolve(checker::check) } }

        assertEquals(3, server.requestsTo(MANIFEST_PATH))
    }

    @Test
    fun `a check that finds no newer version is remembered`() {
        server.serve(MANIFEST_PATH, anUpdateManifest(versionCode = 100).asJson().toByteArray())

        repeat(3) { cache.resolve(checker::check) }

        assertEquals(1, server.requestsTo(MANIFEST_PATH))
    }

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
