package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReply
import org.junit.Assert.assertEquals
import org.junit.Test

class AServerReplyIsParsedIntoADiscoveredServerTest {

    private fun parse(text: String, host: String = HOST): DiscoveredServer? =
        DiscoveryReply.parse(text.toByteArray(Charsets.UTF_8), host)

    @Test
    fun `a well formed reply carries the name the port and the host it came from`() {
        val server = parse(aServerReply("Gaming PC", 4000), "192.168.0.220")

        assertEquals(DiscoveredServer("Gaming PC", "192.168.0.220", 4000), server)
    }

    @Test
    fun `the stream address joins the host and the advertised port`() {
        val server = requireNotNull(parse(aServerReply(port = 4000), "192.168.0.220"))

        assertEquals("192.168.0.220:4000", server.address)
    }

    @Test
    fun `unknown extra fields are ignored`() {
        val text = """{"type":"server","version":1,"name":"Gaming PC","port":3278,"os":"windows","tags":[1,2],"extra":{"a":1}}"""

        assertEquals(DiscoveredServer("Gaming PC", HOST, 3278), parse(text))
    }

    @Test
    fun `any version is accepted`() {
        listOf("0", "2", "99", "\"next\"", "null").forEach { version ->
            val text = """{"type":"server","version":$version,"name":"Gaming PC","port":3278}"""

            assertEquals(version, DiscoveredServer("Gaming PC", HOST, 3278), parse(text))
        }
    }

    @Test
    fun `a missing version is accepted`() {
        val text = """{"type":"server","name":"Gaming PC","port":3278}"""

        assertEquals(DiscoveredServer("Gaming PC", HOST, 3278), parse(text))
    }

    @Test
    fun `the order of the fields and the spaces between them do not matter`() {
        val text = """ { "port" : 3278 , "name" : "Gaming PC" , "type" : "server" } """

        assertEquals(DiscoveredServer("Gaming PC", HOST, 3278), parse(text))
    }

    @Test
    fun `the lowest and the highest port are accepted`() {
        assertEquals(1, parse(aServerReply(port = 1))?.port)
        assertEquals(65535, parse(aServerReply(port = 65535))?.port)
    }

    @Test
    fun `a name in another script survives the trip`() {
        assertEquals("Игровой ПК", parse(aServerReply(name = "Игровой ПК"))?.name)
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
