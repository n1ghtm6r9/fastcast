package dev.fastcast.client.connect

import dev.fastcast.client.testing.aServer
import org.junit.Assert.assertNull
import org.junit.Test

class AnEmptyListHasNothingToFocusTest {

    private val focus = ServerFocus()

    @Test
    fun `nothing is focused before any server answers`() {
        assertNull(focus.indexToFocus(emptyList(), "Gaming PC"))
    }

    @Test
    fun `nothing is focused after the last server leaves`() {
        focus.userMoved()
        focus.focusedOn(aServer(name = "Gaming PC"), 0)

        assertNull(focus.indexToFocus(emptyList(), "Gaming PC"))
    }
}
