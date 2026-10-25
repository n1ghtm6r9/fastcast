package dev.fastcast.client.decoder

import android.media.MediaFormat

class OutputColor(
    val standard: Int?,
    val range: Int?,
    val transfer: Int?,
) {

    companion object {
        val NONE = OutputColor(standard = null, range = null, transfer = null)

        fun of(format: MediaFormat): OutputColor = OutputColor(
            standard = integerOrNull(format, MediaFormat.KEY_COLOR_STANDARD),
            range = integerOrNull(format, MediaFormat.KEY_COLOR_RANGE),
            transfer = integerOrNull(format, MediaFormat.KEY_COLOR_TRANSFER),
        )

        private fun integerOrNull(format: MediaFormat, key: String): Int? =
            if (format.containsKey(key)) format.getInteger(key) else null
    }
}
