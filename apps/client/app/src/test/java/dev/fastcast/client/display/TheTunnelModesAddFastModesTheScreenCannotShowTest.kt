package dev.fastcast.client.display

import dev.fastcast.client.testing.aDisplayMode
import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelModesAddFastModesTheScreenCannotShowTest {

    private val fullHdSixty = aDisplayMode(modeId = 1, width = 1920, height = 1080, refreshRate = 60f)
    private val uhdSixty = aDisplayMode(modeId = 2, refreshRate = 60f)
    private val fullHdFast = aDisplayMode(modeId = 4, width = 1920, height = 1080, refreshRate = 120f)
    private val anySize = { _: Int, _: Int, _: Int -> true }
    private val noSize = { _: Int, _: Int, _: Int -> false }
    private val upToQuadHd = { width: Int, height: Int, fps: Int -> fps == 120 && width * height <= 2560 * 1440 }

    @Test
    fun `a sixty hertz screen gets every size its decoder can tunnel at 120`() {
        val offer = ScreenModes.withTunnels(listOf(uhdSixty), declares = upToQuadHd, allows = noSize)

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
    fun `a size the decoder allows but does not declare at 120 is offered as best effort`() {
        val offer = ScreenModes.withTunnels(listOf(uhdSixty), declares = upToQuadHd, allows = anySize)

        assertEquals(
            listOf(
                DisplayModeOption(2, 1920, 1080, 120f, tunneled = true),
                DisplayModeOption(2, 2560, 1440, 120f, tunneled = true),
                uhdSixty,
                DisplayModeOption(2, 3840, 2160, 120f, tunneled = true, bestEffort = true),
            ),
            offer,
        )
    }

    @Test
    fun `a size the decoder only allows is still offered when it declares nothing`() {
        val tunnels = ScreenModes.withTunnels(listOf(uhdSixty), declares = noSize, allows = anySize).filter { it.tunneled }

        assertEquals(listOf(true, true, true), tunnels.map { it.bestEffort })
    }

    @Test
    fun `a size the screen already shows at 120 gets no tunnel mode`() {
        val offer = ScreenModes.withTunnels(listOf(fullHdFast, uhdSixty), declares = anySize, allows = anySize)

        assertEquals(listOf(fullHdFast), offer.filter { it.width == 1920 })
    }

    @Test
    fun `no tunnel mode is larger than the screen`() {
        assertEquals(
            listOf(fullHdSixty, DisplayModeOption(1, 1920, 1080, 120f, tunneled = true)),
            ScreenModes.withTunnels(listOf(fullHdSixty), declares = anySize, allows = anySize),
        )
    }

    @Test
    fun `a tunnel mode keeps the screen in its largest mode`() {
        val tunnels = ScreenModes.withTunnels(listOf(fullHdSixty, uhdSixty), declares = anySize, allows = anySize)
            .filter { it.tunneled }

        assertEquals(setOf(2), tunnels.map { it.modeId }.toSet())
    }

    @Test
    fun `no screen mode means no tunnel mode`() {
        assertEquals(
            emptyList<DisplayModeOption>(),
            ScreenModes.withTunnels(emptyList(), declares = anySize, allows = anySize),
        )
    }

    @Test
    fun `a tunnel mode is saved apart from a screen mode of the same size and rate`() {
        assertEquals("2560x1440@120 tunneled", DisplayModeOption(2, 2560, 1440, 120f, tunneled = true).label)
        assertEquals("2560x1440@120", DisplayModeOption(5, 2560, 1440, 120f).label)
    }

    @Test
    fun `a best effort tunnel mode is saved under the same name as a declared one`() {
        assertEquals(
            "3840x2160@120 tunneled",
            DisplayModeOption(2, 3840, 2160, 120f, tunneled = true, bestEffort = true).label,
        )
    }
}
