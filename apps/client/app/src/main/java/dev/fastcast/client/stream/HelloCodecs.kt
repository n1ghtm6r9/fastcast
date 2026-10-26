package dev.fastcast.client.stream

object HelloCodecs {

    private const val AVC = "avc"
    private const val HEVC = "hevc"
    private const val AVC_ID = 1
    private const val HEVC_ID = 2
    private const val EIGHT_BITS = 8
    private const val TEN_BITS = 10

    fun of(names: List<String>, tenBit: Set<String> = emptySet()): IntArray =
        names.flatMap { name -> idsOf(name, if (name in tenBit) TEN_BITS else EIGHT_BITS) }.toIntArray()

    private fun idsOf(name: String, bitDepth: Int): List<Int> = when (name) {
        AVC -> listOf(AVC_ID, bitDepth)
        HEVC -> listOf(HEVC_ID, bitDepth)
        else -> emptyList()
    }
}
