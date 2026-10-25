package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertNull
import org.junit.Test

class ASameOrOlderManifestIsNotReportedByTheCheckerTest {

    private val server = FakeUpdateServer()

    @After
    fun tearDown() = server.close()

    @Test
    fun `a manifest of the installed build is not an update`() {
        server.serve(MANIFEST_PATH, anUpdateManifest(versionCode = 101).asJson().toByteArray())

        assertNull(checkerInstalled(101).check())
    }

    @Test
    fun `a manifest one build behind is not an update`() {
        server.serve(MANIFEST_PATH, anUpdateManifest(versionCode = 100).asJson().toByteArray())

        assertNull(checkerInstalled(101).check())
    }

    @Test
    fun `a manifest far behind is not an update`() {
        server.serve(MANIFEST_PATH, anUpdateManifest(versionCode = 1).asJson().toByteArray())

        assertNull(checkerInstalled(500).check())
    }

    private fun checkerInstalled(versionCode: Int) =
        UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), versionCode)

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
    }
}
