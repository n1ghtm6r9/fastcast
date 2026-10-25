package dev.fastcast.client.update

import dev.fastcast.client.testing.anUpdateManifestJson
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class OnlyAFullSha256DigestIsAcceptedInAManifestTest {

    @Test
    fun `sixty four hex digits are accepted`() {
        assertEquals("0123456789abcdef".repeat(4), parseWithDigest("0123456789abcdef".repeat(4))?.sha256)
    }

    @Test
    fun `sixty three hex digits are rejected`() {
        assertNull(parseWithDigest("a".repeat(63)))
    }

    @Test
    fun `sixty five hex digits are rejected`() {
        assertNull(parseWithDigest("a".repeat(65)))
    }

    @Test
    fun `a digest with a character that is not hex is rejected`() {
        assertNull(parseWithDigest("a".repeat(63) + "g"))
    }

    @Test
    fun `a digest with leading whitespace is rejected`() {
        assertNull(parseWithDigest(" " + "a".repeat(64)))
    }

    @Test
    fun `a digest with a trailing line break is rejected`() {
        assertNull(parseWithDigest("a".repeat(64) + "\n"))
    }

    @Test
    fun `an empty digest is rejected`() {
        assertNull(parseWithDigest(""))
    }

    @Test
    fun `a digest with a hex prefix is rejected`() {
        assertNull(parseWithDigest("0x" + "a".repeat(62)))
    }

    private fun parseWithDigest(digest: String): UpdateManifest? =
        UpdateManifest.parse(anUpdateManifestJson("sha256" to digest))
}
