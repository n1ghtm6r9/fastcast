package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifest
import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertEquals
import org.junit.Test

class AValidManifestIsParsedTest {

    @Test
    fun `every field of the release manifest is read`() {
        val text = "{\"versionCode\":101,\"versionName\":\"0.1.101\"," +
            "\"url\":\"https://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.101/fastcast-tv.apk\"," +
            "\"sha256\":\"${"ab".repeat(32)}\",\"size\":4321000}"

        assertEquals(
            UpdateManifest(
                versionCode = 101,
                versionName = "0.1.101",
                url = "https://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.101/fastcast-tv.apk",
                sha256 = "ab".repeat(32),
                size = 4_321_000L,
            ),
            UpdateManifest.parse(text),
        )
    }

    @Test
    fun `fields the client does not know are ignored`() {
        val text = anUpdateManifestJson("notes" to "a fix", "build" to 7)

        assertEquals(anUpdateManifest(), UpdateManifest.parse(text))
    }

    @Test
    fun `whitespace around the document is harmless`() {
        val text = "\n  " + anUpdateManifestJson() + "  \r\n"

        assertEquals(anUpdateManifest(), UpdateManifest.parse(text))
    }

    @Test
    fun `an upper case digest is stored in lower case`() {
        val text = anUpdateManifestJson("sha256" to "AB".repeat(32))

        assertEquals("ab".repeat(32), UpdateManifest.parse(text)?.sha256)
    }

    @Test
    fun `a mixed case digest is stored in lower case`() {
        val text = anUpdateManifestJson("sha256" to "aB1".repeat(21) + "c")

        assertEquals("ab1".repeat(21) + "c", UpdateManifest.parse(text)?.sha256)
    }

    @Test
    fun `a size beyond what fits in an int is accepted`() {
        val text = anUpdateManifestJson("size" to 5_000_000_000L)

        assertEquals(5_000_000_000L, UpdateManifest.parse(text)?.size)
    }

    @Test
    fun `the largest version code is accepted`() {
        val text = anUpdateManifestJson("versionCode" to Int.MAX_VALUE)

        assertEquals(Int.MAX_VALUE, UpdateManifest.parse(text)?.versionCode)
    }

    @Test
    fun `a url with a query is accepted`() {
        val url = "https://objects.githubusercontent.com/github-production-release-asset/1?X-Amz-Expires=300&b=c"

        assertEquals(url, UpdateManifest.parse(anUpdateManifestJson("url" to url))?.url)
    }
}
