package dev.fastcast.client.stream

import kotlin.random.Random

object NativeStreamLink : StreamLink {

    private const val NO_MODE = 0

    override fun open(target: StreamTarget): Long {
        val request = target.request
        return try {
            NativeLink.open(
                target.host,
                target.port,
                Random.nextInt(),
                request?.width ?: NO_MODE,
                request?.height ?: NO_MODE,
                request?.fps ?: NO_MODE,
                request?.let { HelloCodecs.of(it.codecs, it.tenBitCodecs) } ?: IntArray(0),
                request?.noise == true,
            )
        } catch (e: LinkageError) {
            NativeLink.NO_HANDLE
        }
    }

    override fun receive(link: Long, timeoutMs: Int): ByteArray? = NativeLink.receive(link, timeoutMs)

    override fun sendStats(link: Long, text: ByteArray): Boolean = NativeLink.sendStats(link, text)

    override fun sendNotice(link: Long, text: ByteArray): Boolean = NativeLink.sendNotice(link, text)

    override fun receiveBufferBytes(link: Long): Int = NativeLink.receiveBufferBytes(link)

    override fun socketDrops(link: Long): Int = NativeLink.socketDrops(link)

    override fun inboxDrops(link: Long): Int = NativeLink.inboxDrops(link)

    override fun takeLongestPauseMs(link: Long): Int = NativeLink.takeLongestPauseMs(link)

    override fun requestKeyFrame(link: Long) = NativeLink.requestKeyFrame(link)

    override fun close(link: Long) = NativeLink.close(link)
}
