package dev.fastcast.client.display

import kotlin.math.abs

object DisplayModeChooser {

    private const val TARGET_REFRESH_RATE = 60f
    private const val REFRESH_RATE_TOLERANCE = 0.5f

    fun choose(modes: List<DisplayModeOption>): DisplayModeOption? {
        val largestPicture = modes.maxOfOrNull { it.pixelCount } ?: return null
        return modes
            .filter { it.pixelCount == largestPicture && distanceToTarget(it) < REFRESH_RATE_TOLERANCE }
            .minByOrNull(::distanceToTarget)
    }

    private fun distanceToTarget(mode: DisplayModeOption): Float = abs(mode.refreshRate - TARGET_REFRESH_RATE)
}
