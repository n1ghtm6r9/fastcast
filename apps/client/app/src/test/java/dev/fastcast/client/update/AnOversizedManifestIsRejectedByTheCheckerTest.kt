package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.asJson
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AnOversizedManifestIsRejectedByTheCheckerTest {

    private val server = FakeUpdateServer()
    private val manifest = anUpdateManifest()
    private val checker = UpdateChecker(UpdateHttpClient.create(), server.urlOf(MANIFEST_PATH), 100)

    @After
    fun tearDown() = server.close()

    @Test
    fun `a manifest of exactly sixteen kilobytes is read`() {
        server.serve(MANIFEST_PATH, paddedTo(SIXTEEN_KILOBYTES))

        assertEquals(manifest, checker.check())
    }

    @Test
    fun `a manifest one byte over sixteen kilobytes is rejected`() {
        server.serve(MANIFEST_PATH, paddedTo(SIXTEEN_KILOBYTES + 1))

        assertNull(checker.check())
    }

    @Test
    fun `a very large answer is rejected`() {
        server.serve(MANIFEST_PATH, paddedTo(ONE_MEGABYTE))

        assertNull(checker.check())
    }

    private fun paddedTo(totalBytes: Int): ByteArray {
        val json = manifest.asJson()
        return (json + " ".repeat(totalBytes - json.length)).toByteArray()
    }

    private companion object {
        const val MANIFEST_PATH = "/latest/download/fastcast-tv.json"
        const val SIXTEEN_KILOBYTES = 16 * 1024
        const val ONE_MEGABYTE = 1024 * 1024
    }
}
