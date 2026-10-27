package dev.fastcast.client.stream

object NativeLink {

    const val EVENT_VIDEO = 0
    const val EVENT_CONFIG = 1
    const val EVENT_CONNECTED = 2
    const val EVENT_ENDED = 3
    const val EVENT_NOTICE = 4

    const val NO_HANDLE = 0L

    private const val LIBRARY = "fastcast_native"

    init {
        System.loadLibrary(LIBRARY)
    }

    @JvmStatic
    external fun protocolVersion(): Int

    @JvmStatic
    external fun open(
        host: String,
        port: Int,
        nonce: Int,
        width: Int,
        height: Int,
        fps: Int,
        codecs: IntArray,
    ): Long

    @JvmStatic
    external fun receive(link: Long, timeoutMs: Int): ByteArray?

    @JvmStatic
    external fun sendStats(link: Long, text: ByteArray): Boolean

    @JvmStatic
    external fun sendNotice(link: Long, text: ByteArray): Boolean

    @JvmStatic
    external fun receiveBufferBytes(link: Long): Int

    @JvmStatic
    external fun requestKeyFrame(link: Long)

    @JvmStatic
    external fun close(link: Long)
}
