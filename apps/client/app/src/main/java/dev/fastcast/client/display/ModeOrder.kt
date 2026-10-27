package dev.fastcast.client.display

object ModeOrder {

    val SMALLEST_FIRST: Comparator<DisplayModeOption> =
        compareBy<DisplayModeOption> { it.pixelCount }.thenBy { it.refreshRate }
}
