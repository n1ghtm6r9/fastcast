package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheRefreshRateClosestToSixtyHertzIsChosenTest {

    @Test
    fun `exactly sixty hertz wins over fifty nine point nine four`() {
        val ntsc = aDisplayMode(modeId = 1, refreshRate = 59.94f)
        val exact = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(exact, DisplayModeChooser.choose(listOf(ntsc, exact)) { false })
    }

    @Test
    fun `the winner does not depend on the order of the modes`() {
        val ntsc = aDisplayMode(modeId = 1, refreshRate = 59.94f)
        val exact = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(exact, DisplayModeChooser.choose(listOf(exact, ntsc)) { false })
    }

    @Test
    fun `fifty nine point nine four is chosen when it is the closest`() {
        val ntsc = aDisplayMode(modeId = 1, refreshRate = 59.94f)
        val others = listOf(24f, 30f, 50f, 120f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 2, refreshRate = rate)
        }

        assertEquals(ntsc, DisplayModeChooser.choose(listOf(ntsc) + others) { false })
    }

    @Test
    fun `one hundred twenty hertz is passed over for sixty`() {
        val fast = aDisplayMode(modeId = 1, refreshRate = 120f)
        val sixty = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(sixty, DisplayModeChooser.choose(listOf(fast, sixty)) { false })
    }

    @Test
    fun `the first of two identical rates wins`() {
        val first = aDisplayMode(modeId = 1, refreshRate = 60f)
        val second = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(first, DisplayModeChooser.choose(listOf(first, second)) { false })
    }

    @Test
    fun `a rate just inside the half hertz window is accepted`() {
        val above = aDisplayMode(modeId = 1, refreshRate = 60.49f)
        val below = aDisplayMode(modeId = 2, refreshRate = 59.51f)

        assertEquals(above, DisplayModeChooser.choose(listOf(above)) { false })
        assertEquals(below, DisplayModeChooser.choose(listOf(below)) { false })
    }

    @Test
    fun `the closer of two rates inside the window wins`() {
        val near = aDisplayMode(modeId = 1, refreshRate = 59.9f)
        val far = aDisplayMode(modeId = 2, refreshRate = 60.4f)

        assertEquals(near, DisplayModeChooser.choose(listOf(far, near)) { false })
    }
}
