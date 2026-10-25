package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertEquals
import org.junit.Test

class TheFocusStaysOnTheSameServerWhenTheListChangesTest {

    private val focus = ServerFocus()
    private val laptop = aServer(name = "Laptop", host = "192.168.0.11")
    private val office = aServer(name = "Office", host = "192.168.0.12")
    private val gamingPc = aServer(name = "Gaming PC", host = "192.168.0.13")

    @Test
    fun `a server that joins later does not move the focus`() {
        focus.userMoved()
        focus.focusedOn(office, 1)

        assertEquals(1, focus.indexToFocus(listOf(laptop, office, gamingPc), null))
    }

    @Test
    fun `the focus follows its server when a server before it leaves`() {
        focus.userMoved()
        focus.focusedOn(office, 1)

        assertEquals(0, focus.indexToFocus(listOf(office, gamingPc), null))
    }

    @Test
    fun `a renamed server keeps the focus`() {
        focus.userMoved()
        focus.focusedOn(office, 1)
        val renamed = aServer(name = "Office 2", host = "192.168.0.12")

        assertEquals(0, focus.indexToFocus(listOf(renamed, gamingPc, laptop), null))
    }
}
