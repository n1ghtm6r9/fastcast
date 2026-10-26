package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheLargestPictureIsChosenBeforeTheRefreshRateTest {

    @Test
    fun `a larger picture wins over a smaller one at the same refresh rate`() {
        val small = aDisplayMode(modeId = 1, width = 1920, height = 1080)
        val large = aDisplayMode(modeId = 2, width = 3840, height = 2160)

        assertEquals(large, DisplayModeChooser.choose(listOf(small, large)) { false })
    }

    @Test
    fun `the order of the modes does not matter`() {
        val small = aDisplayMode(modeId = 1, width = 1920, height = 1080)
        val large = aDisplayMode(modeId = 2, width = 3840, height = 2160)

        assertEquals(large, DisplayModeChooser.choose(listOf(large, small)) { false })
    }

    @Test
    fun `the pictures are compared by pixel count and not by width alone`() {
        val wide = aDisplayMode(modeId = 1, width = 4096, height = 1716)
        val tall = aDisplayMode(modeId = 2, width = 3840, height = 2160)

        assertEquals(tall, DisplayModeChooser.choose(listOf(wide, tall)) { false })
    }

    @Test
    fun `a smaller picture is never chosen even when only it runs at sixty hertz`() {
        val largeButSlow = aDisplayMode(modeId = 1, width = 3840, height = 2160, refreshRate = 30f)
        val smallAndFast = aDisplayMode(modeId = 2, width = 1920, height = 1080, refreshRate = 60f)

        assertNull(DisplayModeChooser.choose(listOf(largeButSlow, smallAndFast)) { false })
    }

    @Test
    fun `the largest picture decides first and its best refresh rate second`() {
        val largeAtSixty = aDisplayMode(modeId = 1, width = 3840, height = 2160, refreshRate = 59.94f)
        val smallAtSixty = aDisplayMode(modeId = 2, width = 1920, height = 1080, refreshRate = 60f)
        val largeAtThirty = aDisplayMode(modeId = 3, width = 3840, height = 2160, refreshRate = 30f)

        assertEquals(largeAtSixty, DisplayModeChooser.choose(listOf(smallAtSixty, largeAtThirty, largeAtSixty)) { false })
    }

    @Test
    fun `a single suitable mode is chosen`() {
        val only = aDisplayMode(modeId = 7, width = 1280, height = 720, refreshRate = 60f)

        assertEquals(only, DisplayModeChooser.choose(listOf(only)) { false })
    }
}
