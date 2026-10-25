package dev.fastcast.client.discovery

import org.junit.Assert.assertNull
import org.junit.Test

class AReplyWithAnInvalidPortIsRejectedTest {

    private fun parse(port: String): DiscoveredServer? {
        val text = """{"type":"server","version":1,"name":"Gaming PC","port":$port}"""
        return DiscoveryReply.parse(text.toByteArray(Charsets.UTF_8), HOST)
    }

    @Test
    fun `a port below the valid range is rejected`() {
        listOf("0", "-1", "-3278").forEach { port -> assertNull(port, parse(port)) }
    }

    @Test
    fun `a port above the valid range is rejected`() {
        listOf("65536", "70000", "99999", "4294967297").forEach { port -> assertNull(port, parse(port)) }
    }

    @Test
    fun `a port that is not a number is rejected`() {
        listOf("\"3278\"", "\"abc\"", "\"\"", "null", "true", "[]", "{}", "[3278]").forEach { port ->
            assertNull(port, parse(port))
        }
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
