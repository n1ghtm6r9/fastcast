package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertNull
import org.junit.Test

class ADisplayWithoutAModeNearSixtyHertzKeepsItsCurrentModeTest {

    @Test
    fun `no mode is chosen from an empty list`() {
        assertNull(DisplayModeChooser.choose(emptyList()))
    }

    @Test
    fun `no mode is chosen when the largest picture runs at other rates only`() {
        val modes = listOf(24f, 30f, 50f, 120f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 1, refreshRate = rate)
        }

        assertNull(DisplayModeChooser.choose(modes))
    }

    @Test
    fun `sixty point five hertz is outside the window`() {
        assertNull(DisplayModeChooser.choose(listOf(aDisplayMode(refreshRate = 60.5f))))
    }

    @Test
    fun `fifty nine point five hertz is outside the window`() {
        assertNull(DisplayModeChooser.choose(listOf(aDisplayMode(refreshRate = 59.5f))))
    }

    @Test
    fun `fifty hertz is not close enough to sixty`() {
        assertNull(DisplayModeChooser.choose(listOf(aDisplayMode(refreshRate = 50f))))
    }
}
