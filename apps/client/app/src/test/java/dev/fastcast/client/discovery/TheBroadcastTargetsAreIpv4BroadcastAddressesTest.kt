package dev.fastcast.client.discovery

import java.net.Inet4Address
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheBroadcastTargetsAreIpv4BroadcastAddressesTest {

    private val targets = BroadcastTargets.find()

    @Test
    fun `the limited broadcast address comes first`() {
        assertEquals("255.255.255.255", targets.first().hostAddress)
    }

    @Test
    fun `every target is an ipv4 address`() {
        assertTrue(targets.all { it is Inet4Address })
    }

    @Test
    fun `no target is listed twice`() {
        assertEquals(targets.size, targets.toSet().size)
    }

    @Test
    fun `no target is a loopback address`() {
        assertFalse(targets.any { it.isLoopbackAddress })
    }
}
