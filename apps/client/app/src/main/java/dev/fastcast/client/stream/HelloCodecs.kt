package dev.fastcast.client.stream

object HelloCodecs {

    private const val AVC = "avc"
    private const val HEVC = "hevc"
    private const val AVC_ID = 1
    private const val HEVC_ID = 2
    private const val MAX_BIT_DEPTH = 8

    fun of(names: List<String>): IntArray = names.flatMap(::idsOf).toIntArray()

    private fun idsOf(name: String): List<Int> = when (name) {
        AVC -> listOf(AVC_ID, MAX_BIT_DEPTH)
        HEVC -> listOf(HEVC_ID, MAX_BIT_DEPTH)
        else -> emptyList()
    }
}
