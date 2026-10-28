package dev.fastcast.client.player

import dev.fastcast.client.decoder.SupportedCodecs
import dev.fastcast.client.discovery.DiscoveryProtocol
import dev.fastcast.client.stream.StreamRequest
import dev.fastcast.client.stream.StreamTarget

class StreamOffer(
    private val address: String,
    private val request: StreamRequest?,
) {

    private var codecs = request?.codecs.orEmpty()
    private var tenBitCodecs = request?.tenBitCodecs.orEmpty()
    private var configuredCodec: String? = null
    private var failures = 0

    @Synchronized
    fun target(): StreamTarget = StreamTarget(
        host = address.substringBeforeLast(HOST_PORT_SEPARATOR),
        port = portOfAddress(),
        request = request?.let { StreamRequest(it.width, it.height, it.fps, codecs, tenBitCodecs, it.noise) },
    )

    @Synchronized
    fun codecConfigured(codec: String) {
        configuredCodec = SupportedCodecs.nameFor(codec)
    }

    @Synchronized
    fun decoderFailed() {
        val failed = configuredCodec ?: return
        if (!isFirstChoice(failed) || !hasFallbackFor(failed)) return
        failures++
        if (failures < FAILURES_BEFORE_DROP) return
        if (failed in tenBitCodecs) tenBitCodecs = tenBitCodecs - failed else codecs = codecs - failed
        failures = 0
    }

    @Synchronized
    fun streamHealthy() {
        failures = 0
    }

    private fun isFirstChoice(codec: String): Boolean =
        if (codec == SupportedCodecs.AV1) codec in codecs else codec == autoCodecs().firstOrNull()

    private fun hasFallbackFor(codec: String): Boolean = codec in tenBitCodecs || autoCodecs().any { it != codec }

    private fun autoCodecs(): List<String> = codecs - SupportedCodecs.AV1

    private fun portOfAddress(): Int =
        address.substringAfterLast(HOST_PORT_SEPARATOR, "").toIntOrNull()
            ?.takeIf { it in FIRST_PORT..LAST_PORT }
            ?: DiscoveryProtocol.PORT

    private companion object {
        const val FAILURES_BEFORE_DROP = 2
        const val HOST_PORT_SEPARATOR = ':'
        const val FIRST_PORT = 1
        const val LAST_PORT = 65_535
    }
}
