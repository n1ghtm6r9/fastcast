package dev.fastcast.client.display

import kotlin.math.abs

object DisplayModeChooser {

    private val TARGET_REFRESH_RATES = listOf(120f, 60f)
    private const val FALLBACK_REFRESH_RATE = 60f
    private const val REFRESH_RATE_TOLERANCE = 0.5f

    fun choose(modes: List<DisplayModeOption>, sustains: (DisplayModeOption) -> Boolean): DisplayModeOption? =
        modes
            .filter { distanceToTarget(it) < REFRESH_RATE_TOLERANCE }
            .sortedWith(
                compareByDescending<DisplayModeOption> { it.pixelCount }
                    .thenByDescending(::targetOf)
                    .thenBy(::distanceToTarget),
            )
            .firstOrNull(sustains)
            ?: closestToFallbackRate(modes)

    private fun closestToFallbackRate(modes: List<DisplayModeOption>): DisplayModeOption? {
        val largestPicture = modes.maxOfOrNull { it.pixelCount } ?: return null
        return modes
            .filter { it.pixelCount == largestPicture && distanceToFallbackRate(it) < REFRESH_RATE_TOLERANCE }
            .minByOrNull(::distanceToFallbackRate)
    }

    private fun targetOf(mode: DisplayModeOption): Float = TARGET_REFRESH_RATES.minBy { abs(mode.refreshRate - it) }

    private fun distanceToTarget(mode: DisplayModeOption): Float = abs(mode.refreshRate - targetOf(mode))

    private fun distanceToFallbackRate(mode: DisplayModeOption): Float = abs(mode.refreshRate - FALLBACK_REFRESH_RATE)
}
