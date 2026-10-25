package dev.fastcast.client.stream

object StreamUrl {

    private const val PREFIX = "ws://"
    private const val PATH = "/ws"
    private const val WIDTH = "width"
    private const val HEIGHT = "height"
    private const val FPS = "fps"
    private const val CODECS = "codecs"
    private const val PARAMETER_SEPARATOR = "&"
    private const val CODEC_SEPARATOR = ","

    fun build(address: String, request: StreamRequest?): String {
        val endpoint = "$PREFIX$address$PATH"
        return if (request == null) endpoint else "$endpoint?${query(request)}"
    }

    private fun query(request: StreamRequest): String = listOfNotNull(
        "$WIDTH=${request.width}",
        "$HEIGHT=${request.height}",
        "$FPS=${request.fps}",
        codecsParameter(request.codecs),
    ).joinToString(PARAMETER_SEPARATOR)

    private fun codecsParameter(codecs: List<String>): String? =
        codecs.takeIf { it.isNotEmpty() }?.joinToString(CODEC_SEPARATOR, prefix = "$CODECS=")
}
