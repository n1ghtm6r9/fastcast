package dev.fastcast.client.discovery

import org.junit.Assert.assertNull
import org.junit.Test

class OnlyAMessageOfTypeServerIsAServerReplyTest {

    private fun parse(text: String): DiscoveredServer? = DiscoveryReply.parse(text.toByteArray(Charsets.UTF_8), HOST)

    @Test
    fun `a probe echoed by another client is rejected`() {
        assertNull(parse("""{"type":"discover","version":1,"name":"Gaming PC","port":3278}"""))
    }

    @Test
    fun `a message of any other type is rejected`() {
        listOf("\"stats\"", "\"config\"", "\"Server\"", "\"SERVER\"", "\" server\"", "\"\"").forEach { type ->
            assertNull(type, parse("""{"type":$type,"version":1,"name":"Gaming PC","port":3278}"""))
        }
    }

    @Test
    fun `a type that is not text is rejected`() {
        listOf("5", "null", "true", "[]", "{}").forEach { type ->
            assertNull(type, parse("""{"type":$type,"version":1,"name":"Gaming PC","port":3278}"""))
        }
    }

    @Test
    fun `a message without a type is rejected`() {
        assertNull(parse("""{"version":1,"name":"Gaming PC","port":3278}"""))
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
