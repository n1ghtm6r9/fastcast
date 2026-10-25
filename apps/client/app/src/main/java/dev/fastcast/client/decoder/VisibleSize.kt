package dev.fastcast.client.decoder

import android.media.MediaFormat

object VisibleSize {

    private const val CROP_LEFT = "crop-left"
    private const val CROP_RIGHT = "crop-right"
    private const val CROP_TOP = "crop-top"
    private const val CROP_BOTTOM = "crop-bottom"

    fun width(format: MediaFormat): Int = dimension(format, CROP_LEFT, CROP_RIGHT, MediaFormat.KEY_WIDTH)

    fun height(format: MediaFormat): Int = dimension(format, CROP_TOP, CROP_BOTTOM, MediaFormat.KEY_HEIGHT)

    private fun dimension(format: MediaFormat, startKey: String, endKey: String, sizeKey: String): Int = when {
        format.containsKey(startKey) && format.containsKey(endKey) ->
            format.getInteger(endKey) - format.getInteger(startKey) + 1
        format.containsKey(sizeKey) -> format.getInteger(sizeKey)
        else -> 0
    }
}
