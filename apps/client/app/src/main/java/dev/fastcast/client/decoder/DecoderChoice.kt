package dev.fastcast.client.decoder

class DecoderChoice(
    val name: String,
    val supportsLowLatency: Boolean,
) {

    val isQualcomm: Boolean
        get() = QUALCOMM_PREFIXES.any { name.startsWith(it, ignoreCase = true) }

    private companion object {
        val QUALCOMM_PREFIXES = listOf("OMX.qcom.", "OMX.qti.", "c2.qti.")
    }
}
