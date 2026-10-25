package dev.fastcast.client.player

import dev.fastcast.client.decoder.SupportedCodecs
import dev.fastcast.client.stream.StreamRequest
import dev.fastcast.client.stream.StreamUrl

class StreamOffer(
    private val address: String,
    private val request: StreamRequest?,
) {

    private var codecs = request?.codecs.orEmpty()
    private var configuredCodec: String? = null
    private var failures = 0

    @Synchronized
    fun url(): String = StreamUrl.build(address, request?.let { StreamRequest(it.width, it.height, it.fps, codecs) })

    @Synchronized
    fun codecConfigured(codec: String) {
        configuredCodec = SupportedCodecs.nameFor(codec)
    }

    @Synchronized
    fun decoderFailed() {
        val failed = configuredCodec ?: return
        if (failed != codecs.firstOrNull() || codecs.size < 2) return
        failures++
        if (failures < FAILURES_BEFORE_DROP) return
        codecs = codecs - failed
        failures = 0
    }

    @Synchronized
    fun streamHealthy() {
        failures = 0
    }

    private companion object {
        const val FAILURES_BEFORE_DROP = 2
    }
}
