package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class WithoutARememberedServerTheFirstOneIsFocusedTest {

    private val focus = ServerFocus()
    private val servers = listOf(
        aServer(name = "Laptop", host = "192.168.0.11"),
        aServer(name = "Office", host = "192.168.0.12"),
    )

    @Test
    fun `the first server is focused when none was used before`() {
        assertEquals(0, focus.indexToFocus(servers, null))
    }

    @Test
    fun `the first server is focused when the remembered one is not on the network`() {
        assertEquals(0, focus.indexToFocus(servers, "Gaming PC"))
    }
}
