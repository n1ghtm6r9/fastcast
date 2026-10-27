package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheScreenModeStepsThroughTheOfferTest {

    private val fast = aDisplayMode(modeId = 1, refreshRate = 120f)
    private val sixty = aDisplayMode(modeId = 2, refreshRate = 60f)
    private val small = aDisplayMode(modeId = 3, width = 1920, height = 1080, refreshRate = 60f)
    private val offer = listOf(small, sixty, fast)

    @Test
    fun `the next mode follows the current one`() {
        assertEquals(fast, ScreenModes.step(offer, sixty, 1))
    }

    @Test
    fun `the previous mode comes before the current one`() {
        assertEquals(small, ScreenModes.step(offer, sixty, -1))
    }

    @Test
    fun `there is no step past either end`() {
        assertEquals(null, ScreenModes.step(offer, small, -1))
        assertEquals(null, ScreenModes.step(offer, fast, 1))
    }

    @Test
    fun `a current mode outside the offer steps to the best one`() {
        val stranger = aDisplayMode(modeId = 9, width = 1280, height = 720, refreshRate = 50f)

        assertEquals(fast, ScreenModes.step(offer, stranger, 1))
        assertEquals(fast, ScreenModes.step(offer, stranger, -1))
    }
}
