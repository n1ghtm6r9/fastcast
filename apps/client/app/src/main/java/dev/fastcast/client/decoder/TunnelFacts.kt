package dev.fastcast.client.decoder

import java.util.Locale

object TunnelFacts {

    private const val NONE = "none"
    private const val READY = "ready"
    private const val NOT_REPORTED = "not reported"
    private const val MEDIAN = 0.5
    private const val MOST = 0.9
    private const val AFTER_QUEUEING = "frames shown %.1f ms after queueing"
    private const val NOTHING_AFTER_QUEUEING = "no frame shown after queueing"
    private const val LATENESS = " and %.1f ms after their time, 90%% within %.1f ms"
    private const val NOTHING_TIMED = ", no frame timed"
    private const val PER_SECOND = "after queueing per second: %s; after their time per second: %s"

    fun of(stats: DecoderStats, audio: String, schedule: String): List<String> = listOf(
        "tunnel decoder ${stats.decoderName.ifEmpty { NONE }}",
        "first tunnel frame ${if (stats.tunnelFrameReady) READY else NOT_REPORTED}",
        "codec started ${stats.codecStarts.get()} times",
        afterQueueing(stats.delays) + lateness(stats.lateness),
        PER_SECOND.format(Locale.ROOT, DelayTrendText.of(stats.delayTrend), DelayTrendText.of(stats.latenessTrend)),
        audio,
        schedule,
    )

    private fun afterQueueing(delays: RenderDelays): String =
        delays.medianMs()?.let { AFTER_QUEUEING.format(Locale.ROOT, it) } ?: NOTHING_AFTER_QUEUEING

    private fun lateness(lateness: FrameLateness): String {
        val median = lateness.percentileMs(MEDIAN) ?: return NOTHING_TIMED
        val most = lateness.percentileMs(MOST) ?: return NOTHING_TIMED
        return LATENESS.format(Locale.ROOT, median, most)
    }
}
