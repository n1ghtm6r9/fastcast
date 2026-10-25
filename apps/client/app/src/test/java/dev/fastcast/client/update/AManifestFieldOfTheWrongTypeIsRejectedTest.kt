package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.json.JSONObject
import org.junit.Assert.assertNull
import org.junit.Test

class AManifestFieldOfTheWrongTypeIsRejectedTest {

    @Test
    fun `a version code written as text is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to "101")))
    }

    @Test
    fun `a version code with a fraction is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to 101.5)))
    }

    @Test
    fun `a version code that is a boolean is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to true)))
    }

    @Test
    fun `a version code that is null is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to JSONObject.NULL)))
    }

    @Test
    fun `a version name that is a number is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionName" to 101)))
    }

    @Test
    fun `a version name that is null is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionName" to JSONObject.NULL)))
    }

    @Test
    fun `a url that is a number is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("url" to 7)))
    }

    @Test
    fun `a url that is null is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("url" to JSONObject.NULL)))
    }

    @Test
    fun `a digest that is a number is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("sha256" to 7)))
    }

    @Test
    fun `a digest that is null is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("sha256" to JSONObject.NULL)))
    }

    @Test
    fun `a size written as text is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to "4321000")))
    }

    @Test
    fun `a size with a fraction is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to 4_321_000.5)))
    }

    @Test
    fun `a size that is null is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to JSONObject.NULL)))
    }
}
