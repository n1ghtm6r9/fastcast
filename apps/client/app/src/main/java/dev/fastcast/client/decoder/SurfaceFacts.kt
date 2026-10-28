package dev.fastcast.client.decoder

import java.util.Locale

object SurfaceFacts {

    private const val NONE = "none"
    private const val DECODED = "frames decoded %.1f ms after queueing"
    private const val NOTHING_DECODED = "no frame decoded after queueing"
    private const val SHOWN = "frames shown %.1f ms after queueing"
    private const val NOTHING_SHOWN = "no frame shown after queueing"
    private const val PER_SECOND = "shown after queueing per second: %s"

    fun of(stats: DecoderStats): List<String> = listOf(
        "surface decoder ${stats.decoderName.ifEmpty { NONE }}",
        "codec started ${stats.codecStarts.get()} times",
        medianOf(stats.decodeDelays, DECODED) ?: NOTHING_DECODED,
        medianOf(stats.delays, SHOWN) ?: NOTHING_SHOWN,
        PER_SECOND.format(Locale.ROOT, DelayTrendText.of(stats.delayTrend)),
    )

    private fun medianOf(delays: RenderDelays, text: String): String? =
        delays.medianMs()?.let { text.format(Locale.ROOT, it) }
}
