package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheScreenModesOfferEveryDecodableModeTest {

    private val fullHdSixty = aDisplayMode(modeId = 1, width = 1920, height = 1080, refreshRate = 60f)
    private val uhdSixty = aDisplayMode(modeId = 2, refreshRate = 60f)
    private val uhdFast = aDisplayMode(modeId = 3, refreshRate = 120f)
    private val fullHdFast = aDisplayMode(modeId = 4, width = 1920, height = 1080, refreshRate = 144f)

    @Test
    fun `the modes run from the smallest and slowest picture to the largest and fastest`() {
        val offer = ScreenModes.offer(listOf(uhdFast, fullHdSixty, uhdSixty, fullHdFast)) { true }

        assertEquals(listOf(fullHdSixty, fullHdFast, uhdSixty, uhdFast), offer)
    }

    @Test
    fun `modes the decoder cannot sustain are left out`() {
        val offer = ScreenModes.offer(listOf(fullHdSixty, uhdSixty, uhdFast, fullHdFast)) { it.fps <= 60 }

        assertEquals(listOf(fullHdSixty, uhdSixty), offer)
    }

    @Test
    fun `a fractional rate is left out when a whole rate shows the same`() {
        val ntsc = aDisplayMode(modeId = 5, refreshRate = 59.94f)

        assertEquals(listOf(uhdSixty), ScreenModes.offer(listOf(ntsc, uhdSixty)) { true })
        assertEquals(listOf(ntsc), ScreenModes.offer(listOf(ntsc)) { true })
    }

    @Test
    fun `a saved mode is found by its label`() {
        val offer = listOf(uhdFast, uhdSixty)

        assertEquals(uhdSixty, ScreenModes.saved(offer, "3840x2160@60"))
        assertEquals(null, ScreenModes.saved(offer, "1920x1080@60"))
        assertEquals(null, ScreenModes.saved(offer, null))
    }
}
