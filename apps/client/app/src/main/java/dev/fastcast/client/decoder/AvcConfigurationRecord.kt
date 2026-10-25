package dev.fastcast.client.decoder

import java.nio.BufferUnderflowException
import java.nio.ByteBuffer

class AvcConfigurationRecord private constructor(
    override val nalLengthSize: Int,
    val sequenceParameterSets: List<ByteArray>,
    val pictureParameterSets: List<ByteArray>,
) : CodecConfiguration {

    val csd0: ByteArray
        get() = sequenceParameterSets.withStartCodes()

    val csd1: ByteArray
        get() = pictureParameterSets.withStartCodes()

    override val csdBuffers: List<ByteArray>
        get() = listOf(csd0, csd1)

    companion object {
        private const val LENGTH_SIZE_POSITION = 4
        private const val LENGTH_SIZE_MASK = 0x03
        private const val SEQUENCE_PARAMETER_SET_COUNT_MASK = 0x1F
        private const val BYTE_MASK = 0xFF
        private const val SHORT_MASK = 0xFFFF

        fun parse(bytes: ByteArray): AvcConfigurationRecord? = try {
            read(ByteBuffer.wrap(bytes))
        } catch (e: BufferUnderflowException) {
            null
        } catch (e: IllegalArgumentException) {
            null
        }

        private fun read(buffer: ByteBuffer): AvcConfigurationRecord? {
            buffer.position(LENGTH_SIZE_POSITION)
            val nalLengthSize = (buffer.get().toInt() and LENGTH_SIZE_MASK) + 1
            val sequenceCount = buffer.get().toInt() and SEQUENCE_PARAMETER_SET_COUNT_MASK
            val sequenceParameterSets = readParameterSets(buffer, sequenceCount)
            val pictureCount = buffer.get().toInt() and BYTE_MASK
            val pictureParameterSets = readParameterSets(buffer, pictureCount)
            if (sequenceParameterSets.isEmpty() || pictureParameterSets.isEmpty()) return null
            return AvcConfigurationRecord(nalLengthSize, sequenceParameterSets, pictureParameterSets)
        }

        private fun readParameterSets(buffer: ByteBuffer, count: Int): List<ByteArray> = List(count) {
            ByteArray(buffer.short.toInt() and SHORT_MASK).also { buffer.get(it) }
        }
    }
}
