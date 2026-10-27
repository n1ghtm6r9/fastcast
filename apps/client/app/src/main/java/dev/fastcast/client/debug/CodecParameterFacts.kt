package dev.fastcast.client.debug

object CodecParameterFacts {

    private const val CODEC_SPECIFIC_DATA = "csd-"
    private const val NONE = "none"
    private const val SEPARATOR = ", "

    fun of(vendor: Map<String, String>, outputFormat: Map<String, String>): List<String> {
        val format = outputFormat.filterKeys { !it.startsWith(CODEC_SPECIFIC_DATA) }
        val vendorLine = if (vendor.isEmpty()) "vendor parameters: $NONE" else "vendor parameters (${vendor.size}): ${listed(vendor)}"
        val formatLine = "output format: ${if (format.isEmpty()) NONE else listed(format)}"
        return listOf(vendorLine, formatLine)
    }

    private fun listed(values: Map<String, String>): String =
        values.toSortedMap().entries.joinToString(SEPARATOR) { "${it.key}=${it.value}" }
}
