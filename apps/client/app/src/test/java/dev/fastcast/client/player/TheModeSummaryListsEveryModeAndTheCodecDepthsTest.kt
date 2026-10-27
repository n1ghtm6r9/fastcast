package dev.fastcast.client.player

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheModeSummaryListsEveryModeAndTheCodecDepthsTest {

    @Test
    fun `every size is listed largest first with its rates and every codec with its depth`() {
        val modes = listOf(60f, 120f, 59.94f, 24f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 1, refreshRate = rate)
        } + listOf(50f, 144f).mapIndexed { index, rate ->
            aDisplayMode(modeId = index + 10, width = 1920, height = 1080, refreshRate = rate)
        } + aDisplayMode(modeId = 20, width = 2560, height = 1440, refreshRate = 120f)

        assertEquals(
            "modes 3840x2160 120/60/24, 2560x1440 120, 1920x1080 144/50; hevc 10-bit, avc 8-bit",
            ModeSummary.of(modes, listOf("hevc", "avc"), setOf("hevc")),
        )
    }

    @Test
    fun `a display without a hardware codec says so`() {
        assertEquals(
            "modes 3840x2160 60; no hardware codec",
            ModeSummary.of(listOf(aDisplayMode()), emptyList(), emptySet()),
        )
    }
}
