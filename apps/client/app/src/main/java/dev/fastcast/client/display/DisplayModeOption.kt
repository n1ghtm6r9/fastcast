package dev.fastcast.client.display

import kotlin.math.roundToInt

data class DisplayModeOption(
    val modeId: Int,
    val width: Int,
    val height: Int,
    val refreshRate: Float,
) {

    val fps: Int
        get() = refreshRate.roundToInt()

    val pixelCount: Int
        get() = width * height

    val label: String
        get() = "${width}x$height@$fps"
}
