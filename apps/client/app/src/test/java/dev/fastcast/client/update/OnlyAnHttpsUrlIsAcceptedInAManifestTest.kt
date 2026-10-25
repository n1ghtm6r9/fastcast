package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class OnlyAnHttpsUrlIsAcceptedInAManifestTest {

    @Test
    fun `an https url is accepted`() {
        val url = "https://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.102/fastcast-tv.apk"

        assertEquals(url, parseWithUrl(url)?.url)
    }

    @Test
    fun `a plain http url is rejected`() {
        assertNull(parseWithUrl("http://github.com/n1ghtm6r9/fastcast/releases/download/v0.1.101/fastcast-tv.apk"))
    }

    @Test
    fun `an ftp url is rejected`() {
        assertNull(parseWithUrl("ftp://github.com/fastcast-tv.apk"))
    }

    @Test
    fun `a file url is rejected`() {
        assertNull(parseWithUrl("file:///sdcard/fastcast-tv.apk"))
    }

    @Test
    fun `a url without a scheme is rejected`() {
        assertNull(parseWithUrl("github.com/n1ghtm6r9/fastcast/fastcast-tv.apk"))
    }

    @Test
    fun `a url without a host is rejected`() {
        assertNull(parseWithUrl("https://"))
    }

    @Test
    fun `an empty url is rejected`() {
        assertNull(parseWithUrl(""))
    }

    @Test
    fun `a script url is rejected`() {
        assertNull(parseWithUrl("javascript:alert(1)"))
    }

    private fun parseWithUrl(url: String): UpdateManifest? = UpdateManifest.parse(anUpdateManifestJson("url" to url))
}
