package dev.fastcast.client.stream

interface StreamLink {

    fun open(target: StreamTarget): Long

    fun receive(link: Long, timeoutMs: Int): ByteArray?

    fun sendStats(link: Long, text: ByteArray): Boolean

    fun sendNotice(link: Long, text: ByteArray): Boolean

    fun receiveBufferBytes(link: Long): Int

    fun requestKeyFrame(link: Long)

    fun close(link: Long)
}
