package dev.fastcast.client.decoder

object TunnelFacts {

    private const val NONE = "none"
    private const val READY = "ready"
    private const val NOT_REPORTED = "not reported"

    fun of(stats: DecoderStats, audio: String, schedule: String): List<String> = listOf(
        "tunnel decoder ${stats.decoderName.ifEmpty { NONE }}",
        "first tunnel frame ${if (stats.tunnelFrameReady) READY else NOT_REPORTED}",
        "codec started ${stats.codecStarts.get()} times",
        audio,
        schedule,
    )
}
