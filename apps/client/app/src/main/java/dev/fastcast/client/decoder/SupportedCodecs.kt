package dev.fastcast.client.decoder

object SupportedCodecs {

    const val AV1 = "av1"
    const val HEVC = "hevc"

    private val BEST_FIRST = listOf(VideoMime.AV1 to AV1, VideoMime.HEVC to HEVC, VideoMime.AVC to "avc")

    fun among(canDecode: (mime: String) -> Boolean): List<String> =
        BEST_FIRST.filter { (mime, _) -> canDecode(mime) }.map { (_, name) -> name }

    fun nameFor(codec: String): String? {
        val mime = VideoMime.forCodec(codec) ?: return null
        return BEST_FIRST.firstOrNull { (offeredMime, _) -> offeredMime == mime }?.second
    }
}
