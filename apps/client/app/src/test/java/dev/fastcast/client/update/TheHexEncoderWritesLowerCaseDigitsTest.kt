package dev.fastcast.client.update

import dev.fastcast.client.testing.anApkPayload
import dev.fastcast.client.testing.sha256Hex
import java.security.MessageDigest
import org.junit.Assert.assertEquals
import org.junit.Test

class TheHexEncoderWritesLowerCaseDigitsTest {

    @Test
    fun `every byte becomes two digits`() {
        val bytes = byteArrayOf(0x00, 0x0f, 0x10, 0xab.toByte(), 0xff.toByte())

        assertEquals("000f10abff", bytes.toLowerHex())
    }

    @Test
    fun `the letters are lower case`() {
        val bytes = byteArrayOf(0xAB.toByte(), 0xCD.toByte(), 0xEF.toByte())

        assertEquals("abcdef", bytes.toLowerHex())
    }

    @Test
    fun `an empty array gives an empty text`() {
        assertEquals("", byteArrayOf().toLowerHex())
    }

    @Test
    fun `a sha 256 digest gives sixty four digits`() {
        val digest = MessageDigest.getInstance("SHA-256").digest(ByteArray(0))

        assertEquals(64, digest.toLowerHex().length)
    }

    @Test
    fun `the digest of the empty input is the well known one`() {
        val digest = MessageDigest.getInstance("SHA-256").digest(ByteArray(0))

        assertEquals("e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", digest.toLowerHex())
    }

    @Test
    fun `it agrees with an independent hex writer`() {
        val payload = anApkPayload(5_000)

        assertEquals(sha256Hex(payload), MessageDigest.getInstance("SHA-256").digest(payload).toLowerHex())
    }
}
