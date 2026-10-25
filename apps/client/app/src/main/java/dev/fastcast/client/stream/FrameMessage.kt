package dev.fastcast.client.stream

class FrameMessage(
    val data: ByteArray,
    val keyframe: Boolean,
    val timestampMs: Long,
) {

    val payloadOffset: Int
        get() = HEADER_SIZE

    val payloadSize: Int
        get() = data.size - HEADER_SIZE

    val presentationTimeUs: Long
        get() = timestampMs * MICROSECONDS_PER_MILLISECOND

    companion object {
        const val HEADER_SIZE = 5
        private const val KEYFRAME_FLAG = 0x01
        private const val MICROSECONDS_PER_MILLISECOND = 1_000L

        fun parse(data: ByteArray): FrameMessage? {
            if (data.size <= HEADER_SIZE) return null
            val keyframe = (data[0].toInt() and KEYFRAME_FLAG) != 0
            return FrameMessage(data, keyframe, readTimestamp(data))
        }

        private fun readTimestamp(data: ByteArray): Long =
            (data[1].toLong() and 0xFFL) or
                ((data[2].toLong() and 0xFFL) shl 8) or
                ((data[3].toLong() and 0xFFL) shl 16) or
                ((data[4].toLong() and 0xFFL) shl 24)
    }
}
