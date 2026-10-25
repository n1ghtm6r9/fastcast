package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage

object AnnexBConverter {

    private val START_CODE = byteArrayOf(0, 0, 0, 1)
    private const val INVALID = -1
    private const val BYTE_MASK = 0xFF

    fun convert(frame: FrameMessage, nalLengthSize: Int): VideoSample? {
        val nalCount = countNalUnits(frame, nalLengthSize)
        if (nalCount == INVALID) return null
        return if (nalLengthSize == START_CODE.size) {
            rewriteInPlace(frame)
        } else {
            rewriteIntoNewArray(frame, nalLengthSize, nalCount)
        }
    }

    private fun countNalUnits(frame: FrameMessage, lengthSize: Int): Int {
        val data = frame.data
        var position = frame.payloadOffset
        var count = 0
        while (position < data.size) {
            if (data.size - position < lengthSize) return INVALID
            val length = readLength(data, position, lengthSize)
            if (length <= 0 || length > data.size - position - lengthSize) return INVALID
            position += lengthSize + length
            count++
        }
        return count
    }

    private fun rewriteInPlace(frame: FrameMessage): VideoSample {
        val data = frame.data
        var position = frame.payloadOffset
        while (position < data.size) {
            val length = readLength(data, position, START_CODE.size)
            START_CODE.copyInto(data, position)
            position += START_CODE.size + length
        }
        return VideoSample(data, frame.payloadOffset, frame.payloadSize, frame.presentationTimeUs, frame.keyframe)
    }

    private fun rewriteIntoNewArray(frame: FrameMessage, lengthSize: Int, nalCount: Int): VideoSample {
        val data = frame.data
        val output = ByteArray(frame.payloadSize + nalCount * (START_CODE.size - lengthSize))
        var read = frame.payloadOffset
        var write = 0
        while (read < data.size) {
            val length = readLength(data, read, lengthSize)
            START_CODE.copyInto(output, write)
            data.copyInto(output, write + START_CODE.size, read + lengthSize, read + lengthSize + length)
            read += lengthSize + length
            write += START_CODE.size + length
        }
        return VideoSample(output, 0, output.size, frame.presentationTimeUs, frame.keyframe)
    }

    private fun readLength(data: ByteArray, position: Int, lengthSize: Int): Int {
        var length = 0
        for (index in 0 until lengthSize) {
            length = (length shl Byte.SIZE_BITS) or (data[position + index].toInt() and BYTE_MASK)
        }
        return length
    }
}
