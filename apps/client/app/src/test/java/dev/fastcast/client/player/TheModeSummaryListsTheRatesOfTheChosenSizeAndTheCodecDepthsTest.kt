package dev.fastcast.client.player

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheModeSummaryListsTheRatesOfTheChosenSizeAndTheCodecDepthsTest {

    @Test
    fun `the rates of the chosen size and every codec with its depth are listed`() {
        val modes = listOf(120f, 60f, 59.94f, 24f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 1, refreshRate = rate)
        } + aDisplayMode(modeId = 9, width = 1920, height = 1080, refreshRate = 50f)

        assertEquals(
            "3840x2160 modes 120/60/24; hevc 10-bit, avc 8-bit",
            ModeSummary.of(modes, modes.first(), listOf("hevc", "avc"), setOf("hevc")),
        )
    }

    @Test
    fun `a mode without a hardware codec says so`() {
        val mode = aDisplayMode()

        assertEquals("3840x2160 modes 60; no hardware codec", ModeSummary.of(listOf(mode), mode, emptyList(), emptySet()))
    }
}
