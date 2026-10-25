package dev.fastcast.client.decoder

object SupportedCodecs {

    private val BEST_FIRST = listOf(VideoMime.HEVC to "hevc", VideoMime.AVC to "avc")

    fun among(canDecode: (mime: String) -> Boolean): List<String> =
        BEST_FIRST.filter { (mime, _) -> canDecode(mime) }.map { (_, name) -> name }

    fun nameFor(codec: String): String? {
        val mime = VideoMime.forCodec(codec) ?: return null
        return BEST_FIRST.firstOrNull { (offeredMime, _) -> offeredMime == mime }?.second
    }
}
