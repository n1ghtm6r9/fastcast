package dev.fastcast.client.decoder

import java.nio.BufferUnderflowException
import java.nio.ByteBuffer

class HevcConfigurationRecord private constructor(
    override val nalLengthSize: Int,
    val videoParameterSets: List<ByteArray>,
    val sequenceParameterSets: List<ByteArray>,
    val pictureParameterSets: List<ByteArray>,
) : CodecConfiguration {

    val csd0: ByteArray
        get() = (videoParameterSets + sequenceParameterSets + pictureParameterSets).withStartCodes()

    override val csdBuffers: List<ByteArray>
        get() = listOf(csd0)

    companion object {
        private const val LENGTH_SIZE_POSITION = 21
        private const val LENGTH_SIZE_MASK = 0x03
        private const val NAL_UNIT_TYPE_MASK = 0x3F
        private const val VIDEO_PARAMETER_SET_TYPE = 32
        private const val SEQUENCE_PARAMETER_SET_TYPE = 33
        private const val PICTURE_PARAMETER_SET_TYPE = 34
        private const val BYTE_MASK = 0xFF
        private const val SHORT_MASK = 0xFFFF

        fun parse(bytes: ByteArray): HevcConfigurationRecord? = try {
            read(ByteBuffer.wrap(bytes))
        } catch (e: BufferUnderflowException) {
            null
        } catch (e: IllegalArgumentException) {
            null
        }

        private fun read(buffer: ByteBuffer): HevcConfigurationRecord? {
            buffer.position(LENGTH_SIZE_POSITION)
            val nalLengthSize = (buffer.get().toInt() and LENGTH_SIZE_MASK) + 1
            val arrayCount = buffer.get().toInt() and BYTE_MASK
            val arrays = List(arrayCount) { readArray(buffer) }
            val video = nalUnitsOfType(arrays, VIDEO_PARAMETER_SET_TYPE)
            val sequence = nalUnitsOfType(arrays, SEQUENCE_PARAMETER_SET_TYPE)
            val picture = nalUnitsOfType(arrays, PICTURE_PARAMETER_SET_TYPE)
            if (video.isEmpty() || sequence.isEmpty() || picture.isEmpty()) return null
            return HevcConfigurationRecord(nalLengthSize, video, sequence, picture)
        }

        private fun readArray(buffer: ByteBuffer): Pair<Int, List<ByteArray>> {
            val type = buffer.get().toInt() and NAL_UNIT_TYPE_MASK
            val nalCount = buffer.short.toInt() and SHORT_MASK
            return type to List(nalCount) { readNalUnit(buffer) }
        }

        private fun readNalUnit(buffer: ByteBuffer): ByteArray =
            ByteArray(buffer.short.toInt() and SHORT_MASK).also { buffer.get(it) }

        private fun nalUnitsOfType(arrays: List<Pair<Int, List<ByteArray>>>, type: Int): List<ByteArray> =
            arrays.filter { (arrayType, _) -> arrayType == type }.flatMap { (_, nalUnits) -> nalUnits }
    }
}
