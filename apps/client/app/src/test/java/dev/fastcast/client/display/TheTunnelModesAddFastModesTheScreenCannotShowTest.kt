package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelModesAddFastModesTheScreenCannotShowTest {

    private val fullHdSixty = aDisplayMode(modeId = 1, width = 1920, height = 1080, refreshRate = 60f)
    private val uhdSixty = aDisplayMode(modeId = 2, refreshRate = 60f)
    private val fullHdFast = aDisplayMode(modeId = 4, width = 1920, height = 1080, refreshRate = 120f)
    private val anySize = { _: Int, _: Int, _: Int -> true }

    @Test
    fun `a sixty hertz screen gets every size its decoder can tunnel at 120`() {
        val offer = ScreenModes.withTunnels(listOf(uhdSixty)) { width, height, fps ->
            fps == 120 && width * height <= 2560 * 1440
        }

        assertEquals(
            listOf(
                DisplayModeOption(2, 1920, 1080, 120f, tunneled = true),
                DisplayModeOption(2, 2560, 1440, 120f, tunneled = true),
                uhdSixty,
            ),
            offer,
        )
    }

    @Test
    fun `a size the screen already shows at 120 gets no tunnel mode`() {
        val offer = ScreenModes.withTunnels(listOf(fullHdFast, uhdSixty), anySize)

        assertEquals(listOf(fullHdFast), offer.filter { it.width == 1920 })
    }

    @Test
    fun `no tunnel mode is larger than the screen`() {
        assertEquals(
            listOf(fullHdSixty, DisplayModeOption(1, 1920, 1080, 120f, tunneled = true)),
            ScreenModes.withTunnels(listOf(fullHdSixty), anySize),
        )
    }

    @Test
    fun `a tunnel mode keeps the screen in its largest mode`() {
        val tunnels = ScreenModes.withTunnels(listOf(fullHdSixty, uhdSixty), anySize).filter { it.tunneled }

        assertEquals(setOf(2), tunnels.map { it.modeId }.toSet())
    }

    @Test
    fun `no screen mode means no tunnel mode`() {
        assertEquals(emptyList<DisplayModeOption>(), ScreenModes.withTunnels(emptyList(), anySize))
    }

    @Test
    fun `a tunnel mode is saved apart from a screen mode of the same size and rate`() {
        assertEquals("2560x1440@120 tunneled", DisplayModeOption(2, 2560, 1440, 120f, tunneled = true).label)
        assertEquals("2560x1440@120", DisplayModeOption(5, 2560, 1440, 120f).label)
    }
}
