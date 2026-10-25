package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class TheRememberedServerGetsTheFocusFirstTest {

    private val focus = ServerFocus()

    @Test
    fun `the remembered server is focused even when it is not listed first`() {
        val servers = listOf(
            aServer(name = "Laptop", host = "192.168.0.11"),
            aServer(name = "Gaming PC", host = "192.168.0.12"),
            aServer(name = "Office", host = "192.168.0.13"),
        )

        assertEquals(1, focus.indexToFocus(servers, "Gaming PC"))
    }

    @Test
    fun `the remembered server takes the focus when it answers after another one`() {
        val laptop = aServer(name = "Laptop", host = "192.168.0.11")
        val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.12")
        focus.focusedOn(laptop, 0)

        assertEquals(1, focus.indexToFocus(listOf(laptop, gamingPc), "Gaming PC"))
    }

    @Test
    fun `the first of several servers with the remembered name is focused`() {
        val servers = listOf(
            aServer(name = "Laptop", host = "192.168.0.11"),
            aServer(name = "Gaming PC", host = "192.168.0.12"),
            aServer(name = "Gaming PC", host = "192.168.0.13"),
        )

        assertEquals(1, focus.indexToFocus(servers, "Gaming PC"))
    }

    @Test
    fun `the remembered name has to match exactly`() {
        val servers = listOf(
            aServer(name = "gaming pc", host = "192.168.0.11"),
            aServer(name = "Gaming PC 2", host = "192.168.0.12"),
            aServer(name = "Gaming PC", host = "192.168.0.13"),
        )

        assertEquals(2, focus.indexToFocus(servers, "Gaming PC"))
    }
}
