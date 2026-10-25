package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class ALateRememberedServerDoesNotTakeTheFocusFromTheUserTest {

    private val focus = ServerFocus()

    @Test
    fun `the focus stays where the user put it when the remembered server answers`() {
        val laptop = aServer(name = "Laptop", host = "192.168.0.11")
        val office = aServer(name = "Office", host = "192.168.0.12")
        val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.13")
        focus.userMoved()
        focus.focusedOn(office, 1)

        assertEquals(1, focus.indexToFocus(listOf(laptop, office, gamingPc), "Gaming PC"))
    }
}
