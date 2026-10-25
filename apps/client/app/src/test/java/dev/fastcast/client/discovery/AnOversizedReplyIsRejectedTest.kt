package dev.fastcast.client.discovery

import dev.fastcast.client.testing.aServerReplyOfSize
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AnOversizedReplyIsRejectedTest {

    @Test
    fun `a reply of exactly 1024 bytes is accepted`() {
        val reply = aServerReplyOfSize(1024)

        assertEquals(1024, reply.size)
        assertEquals(DiscoveredServer("Gaming PC", HOST, 3278), DiscoveryReply.parse(reply, HOST))
    }

    @Test
    fun `a reply one byte over the limit is rejected`() {
        val reply = aServerReplyOfSize(1025)

        assertEquals(1025, reply.size)
        assertNull(DiscoveryReply.parse(reply, HOST))
    }

    @Test
    fun `a much larger reply is rejected`() {
        assertNull(DiscoveryReply.parse(aServerReplyOfSize(4096), HOST))
        assertNull(DiscoveryReply.parse(aServerReplyOfSize(60_000), HOST))
    }

    private companion object {
        const val HOST = "192.168.0.10"
    }
}
