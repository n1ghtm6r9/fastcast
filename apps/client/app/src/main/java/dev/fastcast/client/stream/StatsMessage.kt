package dev.fastcast.client.stream

import kotlin.math.roundToLong
import org.json.JSONObject

class StatsMessage(
    val decoder: String,
    val mime: String,
    val width: Int,
    val height: Int,
    val fps: Double,
    val mbps: Double,
    val dropped: Long,
    val display: String,
    val colorStandard: Int? = null,
    val colorRange: Int? = null,
    val colorTransfer: Int? = null,
    val error: String? = null,
) {

    fun toJson(): String {
        val json = JSONObject()
            .put(TYPE, TYPE_STATS)
            .put(DECODER, decoder)
            .put(MIME, mime)
            .put(WIDTH, width)
            .put(HEIGHT, height)
            .put(FPS, roundedToTenths(fps))
            .put(MBPS, roundedToTenths(mbps))
            .put(DROPPED, dropped)
            .put(DISPLAY, display)
        colorStandard?.let { json.put(COLOR_STANDARD, it) }
        colorRange?.let { json.put(COLOR_RANGE, it) }
        colorTransfer?.let { json.put(COLOR_TRANSFER, it) }
        error?.let { json.put(ERROR, it) }
        return json.toString()
    }

    private fun roundedToTenths(value: Double): Double = (value * TENTHS_PER_UNIT).roundToLong() / TENTHS_PER_UNIT

    private companion object {
        const val TYPE = "type"
        const val TYPE_STATS = "stats"
        const val DECODER = "decoder"
        const val MIME = "mime"
        const val WIDTH = "width"
        const val HEIGHT = "height"
        const val FPS = "fps"
        const val MBPS = "mbps"
        const val DROPPED = "dropped"
        const val DISPLAY = "display"
        const val COLOR_STANDARD = "colorStandard"
        const val COLOR_RANGE = "colorRange"
        const val COLOR_TRANSFER = "colorTransfer"
        const val ERROR = "error"
        const val TENTHS_PER_UNIT = 10.0
    }
}
