package dev.fastcast.client.stream

class FrameMessage(
    val data: ByteArray,
    val keyframe: Boolean,
    val timestampMs: Long,
    private val offset: Int = 0,
) {

    val payloadOffset: Int
        get() = offset + HEADER_SIZE

    val payloadSize: Int
        get() = data.size - payloadOffset

    val presentationTimeUs: Long
        get() = timestampMs * MICROSECONDS_PER_MILLISECOND

    companion object {
        const val HEADER_SIZE = 5
        private const val KEYFRAME_FLAG = 0x01
        private const val MICROSECONDS_PER_MILLISECOND = 1_000L

        fun parse(data: ByteArray, offset: Int = 0): FrameMessage? {
            if (data.size <= offset + HEADER_SIZE) return null
            val keyframe = (data[offset].toInt() and KEYFRAME_FLAG) != 0
            return FrameMessage(data, keyframe, readTimestamp(data, offset), offset)
        }

        private fun readTimestamp(data: ByteArray, offset: Int): Long =
            (data[offset + 1].toLong() and 0xFFL) or
                ((data[offset + 2].toLong() and 0xFFL) shl 8) or
                ((data[offset + 3].toLong() and 0xFFL) shl 16) or
                ((data[offset + 4].toLong() and 0xFFL) shl 24)
    }
}
