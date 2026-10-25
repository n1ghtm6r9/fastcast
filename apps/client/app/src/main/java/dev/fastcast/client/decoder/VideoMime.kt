package dev.fastcast.client.decoder

object VideoMime {

    const val AVC = "video/avc"
    const val HEVC = "video/hevc"

    private val AVC_PREFIXES = listOf("avc1", "avc3")
    private val HEVC_PREFIXES = listOf("hvc1", "hev1")

    fun forCodec(codec: String): String? = when {
        AVC_PREFIXES.any { codec.startsWith(it, ignoreCase = true) } -> AVC
        HEVC_PREFIXES.any { codec.startsWith(it, ignoreCase = true) } -> HEVC
        else -> null
    }
}
