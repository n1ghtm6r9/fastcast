package dev.fastcast.client.update

import org.junit.Assert.assertNull
import org.junit.Test

class GarbageIsNotAManifestTest {

    @Test
    fun `an empty text is not a manifest`() {
        assertNull(UpdateManifest.parse(""))
    }

    @Test
    fun `a blank text is not a manifest`() {
        assertNull(UpdateManifest.parse("   \n"))
    }

    @Test
    fun `plain text is not a manifest`() {
        assertNull(UpdateManifest.parse("not json"))
    }

    @Test
    fun `an html page is not a manifest`() {
        assertNull(UpdateManifest.parse("<html><body>Not Found</body></html>"))
    }

    @Test
    fun `an unfinished document is not a manifest`() {
        assertNull(UpdateManifest.parse("{\"versionCode\":101,\"versionName\":"))
    }

    @Test
    fun `a json array is not a manifest`() {
        assertNull(UpdateManifest.parse("[]"))
    }

    @Test
    fun `a json null is not a manifest`() {
        assertNull(UpdateManifest.parse("null"))
    }

    @Test
    fun `a bare number is not a manifest`() {
        assertNull(UpdateManifest.parse("101"))
    }

    @Test
    fun `a bare string is not a manifest`() {
        assertNull(UpdateManifest.parse("\"0.1.101\""))
    }
}
