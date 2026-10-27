package dev.fastcast.client.display

import kotlin.math.roundToInt

data class DisplayModeOption(
    val modeId: Int,
    val width: Int,
    val height: Int,
    val refreshRate: Float,
    val tunneled: Boolean = false,
) {

    val fps: Int
        get() = refreshRate.roundToInt()

    val pixelCount: Int
        get() = width * height

    val label: String
        get() = if (tunneled) "$sizeAndRate tunneled" else sizeAndRate

    private val sizeAndRate: String
        get() = "${width}x$height@$fps"
}
