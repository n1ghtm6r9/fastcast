package dev.fastcast.client.stream

interface StreamLink {

    fun open(target: StreamTarget): Long

    fun receive(link: Long, timeoutMs: Int): ByteArray?

    fun sendStats(link: Long, text: ByteArray): Boolean

    fun close(link: Long)
}
