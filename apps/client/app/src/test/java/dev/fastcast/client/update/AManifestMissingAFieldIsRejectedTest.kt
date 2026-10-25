package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertNull
import org.junit.Test

class AManifestMissingAFieldIsRejectedTest {

    @Test
    fun `a manifest without a version code is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to null)))
    }

    @Test
    fun `a manifest without a version name is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionName" to null)))
    }

    @Test
    fun `a manifest without a url is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("url" to null)))
    }

    @Test
    fun `a manifest without a digest is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("sha256" to null)))
    }

    @Test
    fun `a manifest without a size is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to null)))
    }

    @Test
    fun `a manifest without any field is rejected`() {
        assertNull(UpdateManifest.parse("{}"))
    }

    @Test
    fun `a blank version name is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionName" to "  ")))
    }
}
