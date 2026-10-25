package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.Assert.assertNull
import org.junit.Test

class GarbageIsNotAServerReplyTest {

    private fun parse(text: String): DiscoveredServer? = DiscoveryReply.parse(text.toByteArray(Charsets.UTF_8), HOST)

    @Test
    fun `text that is not json is rejected`() {
        listOf("", "   ", "garbage", "{", "}", "server", "null", "[]", "123", "\"server\"").forEach { text ->
            assertNull("\"$text\"", parse(text))
        }
    }

    @Test
    fun `an empty object is rejected`() {
        assertNull(parse("{}"))
    }

    @Test
    fun `an empty datagram is rejected`() {
        assertNull(DiscoveryReply.parse(ByteArray(0), HOST))
    }

    @Test
    fun `bytes that are not text are rejected`() {
        val bytes = byteArrayOf(0xFF.toByte(), 0xFE.toByte(), 0x00, 0x01, 0x80.toByte())

        assertNull(DiscoveryReply.parse(bytes, HOST))
    }

    @Test
    fun `a reply wrapped in an array is rejected`() {
        assertNull(parse("[${aServerReply()}]"))
    }

    @Test
    fun `a reply wrapped in a string is rejected`() {
        assertNull(parse("\"${aServerReply().replace("\"", "\\\"")}\""))
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
