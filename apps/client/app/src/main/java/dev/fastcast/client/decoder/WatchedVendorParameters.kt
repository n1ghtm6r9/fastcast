package dev.fastcast.client.decoder

object WatchedVendorParameters {

    private val NAMES = with(MediaTekParameters) {
        setOf(
            GAME_MODE,
            LOW_LATENCY_MODE,
            FRAME_RATE,
            MAX_FRAME_RATE,
            FORCE_FRAME_RATE,
            FRAME_DROP_THRESHOLD,
            SLOW_DROP,
            SLOW_SYNC,
            MEMC_MODE,
            NON_TUNNEL_RENDER_LATENCY,
            FRAME_RENDERED_INTERVAL,
            START_FLIPPING,
            PICTURE_RESTRICTIONS,
            IVIEW_FORCE_REAL_TIME,
        )
    }

    fun among(supported: List<String>): List<String> = supported.filter(NAMES::contains)
}
