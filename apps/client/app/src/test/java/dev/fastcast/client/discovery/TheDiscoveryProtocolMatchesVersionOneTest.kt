package dev.fastcast.client.discovery

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Test

class TheDiscoveryProtocolMatchesVersionOneTest {

    private fun probeText(): String = String(DiscoveryProtocol.probe(), Charsets.UTF_8)

    @Test
    fun `probes are sent to port 3278`() {
        assertEquals(3278, DiscoveryProtocol.PORT)
    }

    @Test
    fun `the probe is the discover message of version 1`() {
        val json = JSONObject(probeText())

        assertEquals("discover", json.getString("type"))
        assertEquals(1, json.getInt("version"))
        assertEquals(2, json.length())
    }

    @Test
    fun `the probe text is the compact json of the protocol`() {
        assertEquals("""{"type":"discover","version":1}""", probeText())
    }

    @Test
    fun `a reply may be at most 1024 bytes`() {
        assertEquals(1024, DiscoveryProtocol.MAX_REPLY_BYTES)
    }

    @Test
    fun `every probe is a fresh copy`() {
        DiscoveryProtocol.probe().fill(0)

        assertEquals("""{"type":"discover","version":1}""", probeText())
    }
}
