package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ARememberedServerIsChosenAtOnceTest {

    private var nowMs = 0L
    private val policy = AutoConnectPolicy { nowMs }

    @Test
    fun `a remembered server that is the only one is chosen without waiting`() {
        val server = aServer(name = "Gaming PC")

        assertEquals(server, policy.choose(listOf(server), "Gaming PC"))
    }

    @Test
    fun `a remembered server is chosen from among several`() {
        val laptop = aServer(name = "Laptop", host = "192.168.0.11")
        val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.12")
        val office = aServer(name = "Office", host = "192.168.0.13")

        assertEquals(gamingPc, policy.choose(listOf(laptop, gamingPc, office), "Gaming PC"))
    }

    @Test
    fun `the first of several servers with the remembered name is chosen`() {
        val first = aServer(name = "Gaming PC", host = "192.168.0.11")
        val second = aServer(name = "Gaming PC", host = "192.168.0.12")

        assertEquals(first, policy.choose(listOf(first, second), "Gaming PC"))
    }

    @Test
    fun `a remembered server is chosen even when it is not the first one listed`() {
        val stranger = aServer(name = "Stranger", host = "192.168.0.11")
        val remembered = aServer(name = "Gaming PC", host = "192.168.0.12")

        assertEquals(remembered, policy.choose(listOf(stranger, remembered), "Gaming PC"))
    }

    @Test
    fun `the remembered name has to match exactly`() {
        val servers = listOf(
            aServer(name = "gaming pc", host = "192.168.0.11"),
            aServer(name = "Gaming PC 2", host = "192.168.0.12"),
            aServer(name = " Gaming PC", host = "192.168.0.13"),
        )

        assertNull(policy.choose(servers, "Gaming PC"))
    }
}
