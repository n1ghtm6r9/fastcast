package dev.fastcast.client.testing

import dev.fastcast.client.display.DisplayModeOption

fun aDisplayMode(
    modeId: Int = 1,
    width: Int = 3840,
    height: Int = 2160,
    refreshRate: Float = 60f,
): DisplayModeOption = DisplayModeOption(modeId, width, height, refreshRate)
