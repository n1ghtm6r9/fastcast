package dev.fastcast.client.decoder

object VideoMime {

    const val AVC = "video/avc"
    const val HEVC = "video/hevc"
    const val AV1 = "video/av01"

    private val AVC_PREFIXES = listOf("avc1", "avc3")
    private val HEVC_PREFIXES = listOf("hvc1", "hev1")
    private val AV1_PREFIXES = listOf("av01")

    fun forCodec(codec: String): String? = when {
        AVC_PREFIXES.any { codec.startsWith(it, ignoreCase = true) } -> AVC
        HEVC_PREFIXES.any { codec.startsWith(it, ignoreCase = true) } -> HEVC
        AV1_PREFIXES.any { codec.startsWith(it, ignoreCase = true) } -> AV1
        else -> null
    }
}
