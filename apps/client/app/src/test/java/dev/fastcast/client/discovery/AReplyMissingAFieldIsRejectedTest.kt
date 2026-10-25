package dev.fastcast.client.discovery

import org.junit.Assert.assertNull
import org.junit.Test

class AReplyMissingAFieldIsRejectedTest {

    private fun parse(text: String): DiscoveredServer? = DiscoveryReply.parse(text.toByteArray(Charsets.UTF_8), HOST)

    @Test
    fun `a reply without a name is rejected`() {
        assertNull(parse("""{"type":"server","version":1,"port":3278}"""))
    }

    @Test
    fun `a name that is empty or blank is rejected`() {
        listOf("\"\"", "\" \"", "\"   \"", "\"\\t\"").forEach { name ->
            assertNull(name, parse("""{"type":"server","version":1,"name":$name,"port":3278}"""))
        }
    }

    @Test
    fun `a name that is not text is rejected`() {
        listOf("null", "5", "true", "[]", "{}", "[\"Gaming PC\"]").forEach { name ->
            assertNull(name, parse("""{"type":"server","version":1,"name":$name,"port":3278}"""))
        }
    }

    @Test
    fun `a reply without a port is rejected`() {
        assertNull(parse("""{"type":"server","version":1,"name":"Gaming PC"}"""))
    }

    @Test
    fun `a reply with only a type is rejected`() {
        assertNull(parse("""{"type":"server"}"""))
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
