package dev.fastcast.client.display

object TunnelModes {

    private const val FPS = 120
    private val SIZES = listOf(3840 to 2160, 2560 to 1440, 1920 to 1080)

    fun beside(
        offer: List<DisplayModeOption>,
        declares: (width: Int, height: Int, fps: Int) -> Boolean,
        allows: (width: Int, height: Int, fps: Int) -> Boolean,
    ): List<DisplayModeOption> {
        val screen = offer.maxWithOrNull(ModeOrder.SMALLEST_FIRST) ?: return emptyList()
        return SIZES
            .filter { (width, height) -> width <= screen.width && height <= screen.height }
            .filterNot { (width, height) -> offer.any { it.width == width && it.height == height && it.fps >= FPS } }
            .mapNotNull { (width, height) ->
                val declared = declares(width, height, FPS)
                if (!declared && !allows(width, height, FPS)) return@mapNotNull null
                DisplayModeOption(screen.modeId, width, height, FPS.toFloat(), tunneled = true, bestEffort = !declared)
            }
    }
}
