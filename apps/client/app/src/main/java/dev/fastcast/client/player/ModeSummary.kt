package dev.fastcast.client.player

import dev.fastcast.client.display.DisplayModeOption

object ModeSummary {

    private const val NO_CODEC = "no hardware codec"
    private const val TEN_BIT = "10-bit"
    private const val EIGHT_BIT = "8-bit"
    private const val RATE_SEPARATOR = "/"
    private const val SIZE_SEPARATOR = ", "
    private const val CODEC_SEPARATOR = ", "

    fun of(modes: List<DisplayModeOption>, codecs: List<String>, tenBit: Set<String>): String =
        "modes ${sizesOf(modes)}; ${codecsOf(codecs, tenBit)}"

    private fun sizesOf(modes: List<DisplayModeOption>): String = modes
        .groupBy { it.width to it.height }
        .entries
        .sortedByDescending { (size, _) -> size.first * size.second }
        .joinToString(SIZE_SEPARATOR) { (size, sameSize) -> "${size.first}x${size.second} ${ratesOf(sameSize)}" }

    private fun ratesOf(modes: List<DisplayModeOption>): String = modes
        .map { it.fps }
        .distinct()
        .sortedDescending()
        .joinToString(RATE_SEPARATOR)

    private fun codecsOf(codecs: List<String>, tenBit: Set<String>): String =
        if (codecs.isEmpty()) {
            NO_CODEC
        } else {
            codecs.joinToString(CODEC_SEPARATOR) { "$it ${if (it in tenBit) TEN_BIT else EIGHT_BIT}" }
        }
}
