package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AManifestSizeMustBePositiveTest {

    @Test
    fun `a size of zero is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to 0)))
    }

    @Test
    fun `a negative size is rejected`() {
        assertNull(UpdateManifest.parse(anUpdateManifestJson("size" to -1)))
    }

    @Test
    fun `a size of one byte is accepted`() {
        assertEquals(1L, UpdateManifest.parse(anUpdateManifestJson("size" to 1))?.size)
    }
}
