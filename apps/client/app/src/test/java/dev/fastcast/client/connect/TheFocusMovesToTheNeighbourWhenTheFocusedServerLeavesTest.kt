package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class TheFocusMovesToTheNeighbourWhenTheFocusedServerLeavesTest {

    private val focus = ServerFocus()
    private val laptop = aServer(name = "Laptop", host = "192.168.0.11")
    private val office = aServer(name = "Office", host = "192.168.0.12")
    private val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.13")

    @Test
    fun `the next server takes the place of the one that left`() {
        focus.userMoved()
        focus.focusedOn(office, 1)

        assertEquals(1, focus.indexToFocus(listOf(laptop, gamingPc), null))
    }

    @Test
    fun `the last server takes the focus when the focused last one leaves`() {
        focus.userMoved()
        focus.focusedOn(gamingPc, 2)

        assertEquals(1, focus.indexToFocus(listOf(laptop, office), null))
    }
}
