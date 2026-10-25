package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertNull
import org.junit.Test

class AnInvalidVersionCodeIsRejectedTest {

    @Test
    fun `a version code of zero is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to 0)))
    }

    @Test
    fun `a negative version code is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to -1)))
    }

    @Test
    fun `a version code beyond an int is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("versionCode" to Int.MAX_VALUE + 1L)))
    }
}
