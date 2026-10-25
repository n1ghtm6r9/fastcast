package dev.fastcast.client.discovery

object DiscoveryProtocol {
    const val PORT = 3278
    const val MAX_REPLY_BYTES = 1024

    private const val PROBE_TEXT = """{"type":"discover","version":1}"""

    fun probe(): ByteArray = PROBE_TEXT.toByteArray(Charsets.UTF_8)
}
