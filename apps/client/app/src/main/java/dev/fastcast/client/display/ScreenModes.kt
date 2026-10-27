package dev.fastcast.client.display

import kotlin.math.abs

object ScreenModes {

    fun offer(modes: List<DisplayModeOption>, sustains: (DisplayModeOption) -> Boolean): List<DisplayModeOption> = modes
        .filter(sustains)
        .groupBy { it.label }
        .values
        .map { sameLabel -> sameLabel.minBy { abs(it.refreshRate - it.fps) } }
        .sortedWith(compareBy<DisplayModeOption> { it.pixelCount }.thenBy { it.refreshRate })

    fun saved(offer: List<DisplayModeOption>, label: String?): DisplayModeOption? =
        offer.firstOrNull { it.label == label }

    fun step(offer: List<DisplayModeOption>, current: DisplayModeOption?, delta: Int): DisplayModeOption? {
        val index = offer.indexOfFirst { it.label == current?.label }
        return if (index < 0) offer.lastOrNull() else offer.getOrNull(index + delta)
    }
}
