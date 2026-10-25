package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class ADisplayModeReportsItsRefreshRateInWholeHertzTest {

    @Test
    fun `fifty nine point nine four hertz is sixty`() {
        assertEquals(60, aDisplayMode(refreshRate = 59.94f).fps)
    }

    @Test
    fun `one hundred nineteen point eight eight hertz is one hundred twenty`() {
        assertEquals(120, aDisplayMode(refreshRate = 119.88f).fps)
    }

    @Test
    fun `twenty three point nine seven six hertz is twenty four`() {
        assertEquals(24, aDisplayMode(refreshRate = 23.976f).fps)
    }

    @Test
    fun `twenty nine point nine seven hertz is thirty`() {
        assertEquals(30, aDisplayMode(refreshRate = 29.97f).fps)
    }

    @Test
    fun `a whole rate stays as it is`() {
        assertEquals(50, aDisplayMode(refreshRate = 50f).fps)
    }

    @Test
    fun `the label names the picture and the rounded rate`() {
        assertEquals("3840x2160@60", aDisplayMode(width = 3840, height = 2160, refreshRate = 59.94f).label)
    }

    @Test
    fun `the pixel count is width times height`() {
        assertEquals(8_294_400, aDisplayMode(width = 3840, height = 2160).pixelCount)
    }
}
