package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheFastestDecodableModeOfTheLargestPictureIsChosenTest {

    @Test
    fun `one hundred twenty hertz wins when the decoder sustains it`() {
        val sixty = aDisplayMode(modeId = 1, refreshRate = 60f)
        val fast = aDisplayMode(modeId = 2, refreshRate = 120f)

        assertEquals(fast, DisplayModeChooser.choose(listOf(sixty, fast)) { true })
    }

    @Test
    fun `sixty hertz is kept when the decoder cannot sustain one hundred twenty`() {
        val sixty = aDisplayMode(modeId = 1, refreshRate = 60f)
        val fast = aDisplayMode(modeId = 2, refreshRate = 120f)

        assertEquals(sixty, DisplayModeChooser.choose(listOf(fast, sixty)) { it.fps <= 60 })
    }

    @Test
    fun `exactly one hundred twenty wins over one hundred nineteen point eight eight`() {
        val ntsc = aDisplayMode(modeId = 1, refreshRate = 119.88f)
        val exact = aDisplayMode(modeId = 2, refreshRate = 120f)

        assertEquals(exact, DisplayModeChooser.choose(listOf(ntsc, exact)) { true })
        assertEquals(exact, DisplayModeChooser.choose(listOf(exact, ntsc)) { true })
    }

    @Test
    fun `a larger picture at sixty wins over a smaller one at one hundred twenty`() {
        val small = aDisplayMode(modeId = 1, width = 1920, height = 1080, refreshRate = 120f)
        val large = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(large, DisplayModeChooser.choose(listOf(small, large)) { true })
    }

    @Test
    fun `a smaller picture is chosen when no mode of the largest one can be decoded`() {
        val large = aDisplayMode(modeId = 1, refreshRate = 60f)
        val small = aDisplayMode(modeId = 2, width = 1920, height = 1080, refreshRate = 120f)

        assertEquals(small, DisplayModeChooser.choose(listOf(large, small)) { it.width <= 1920 })
    }

    @Test
    fun `rates the server cannot match evenly are skipped`() {
        val uneven = listOf(50f, 100f, 144f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 1, refreshRate = rate)
        }
        val sixty = aDisplayMode(modeId = 4, refreshRate = 60f)

        assertEquals(sixty, DisplayModeChooser.choose(uneven + sixty) { true })
    }

    @Test
    fun `without a decodable mode the largest picture closest to sixty hertz is chosen`() {
        val fast = aDisplayMode(modeId = 1, refreshRate = 120f)
        val sixty = aDisplayMode(modeId = 2, refreshRate = 60f)

        assertEquals(sixty, DisplayModeChooser.choose(listOf(fast, sixty)) { false })
    }
}
