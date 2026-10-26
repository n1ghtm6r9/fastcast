package dev.fastcast.client.stream

import kotlin.random.Random

object NativeStreamLink : StreamLink {

    private const val NO_MODE = 0

    override fun open(target: StreamTarget): Long {
        val request = target.request
        return NativeLink.open(
            target.host,
            target.port,
            Random.nextInt(),
            request?.width ?: NO_MODE,
            request?.height ?: NO_MODE,
            request?.fps ?: NO_MODE,
            request?.let { HelloCodecs.of(it.codecs) } ?: IntArray(0),
        )
    }

    override fun receive(link: Long, timeoutMs: Int): ByteArray? = NativeLink.receive(link, timeoutMs)

    override fun sendStats(link: Long, text: ByteArray): Boolean = NativeLink.sendStats(link, text)

    override fun close(link: Long) = NativeLink.close(link)
}
