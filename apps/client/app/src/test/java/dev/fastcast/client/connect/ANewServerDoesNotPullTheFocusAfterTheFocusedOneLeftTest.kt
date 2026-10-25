package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class ANewServerDoesNotPullTheFocusAfterTheFocusedOneLeftTest {

    private val focus = ServerFocus()
    private val laptop = aServer(name = "Laptop", host = "192.168.0.11")
    private val office = aServer(name = "Office", host = "192.168.0.12")
    private val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.13")

    @Test
    fun `the focus stays on the server that took the place of the one that left`() {
        focus.userMoved()
        focus.focusedOn(office, 1)
        focus.indexToFocus(listOf(laptop), null)

        assertEquals(0, focus.indexToFocus(listOf(laptop, gamingPc), null))
    }

    @Test
    fun `the focus stays on the neighbour when the server that left comes back`() {
        focus.userMoved()
        focus.focusedOn(office, 1)
        focus.indexToFocus(listOf(laptop, gamingPc), null)

        assertEquals(1, focus.indexToFocus(listOf(laptop, gamingPc, office), null))
    }
}
