package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class ANewerManifestIsReportedByTheCheckerTest {

    private val server = FakeUpdateServer()

    @After
    fun tearDown() = server.close()

    @Test
    fun `a manifest newer than the installed build is returned whole`() {
        val manifest = anUpdateManifest(versionCode = 101)
        server.serve(MANIFEST_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, checkerInstalled(100).check())
    }

    @Test
    fun `a manifest many builds ahead is returned`() {
        val manifest = anUpdateManifest(versionCode = 450)
        server.serve(MANIFEST_PATH, manifest.asJson().toByteArray())

        assertEquals(manifest, checkerInstalled(1).check())
    }

    @Test
    fun `the manifest is requested once per check`() {
        server.serve(MANIFEST_PATH, anUpdateManifest().asJson().toByteArray())

        checkerInstalled(100).check()

        assertEquals(1, server.requestsTo(MANIFEST_PATH))
    }

    @Test
    fun `every check asks the server again`() {
        server.serve(MANIFEST_PATH, anUpdateManifest().asJson().toByteArray())
        val checker = checkerInstalled(100)

        checker.check()
        checker.check()

        assertEquals(2, server.requestsTo(MANIFEST_PATH))
    }

    private fun checkerInstalled(versionCode: Int) =
        UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), versionCode)

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
