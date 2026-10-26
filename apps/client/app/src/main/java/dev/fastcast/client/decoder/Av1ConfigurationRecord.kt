package dev.fastcast.client.decoder

class Av1ConfigurationRecord private constructor(
    private val record: ByteArray,
) : CodecConfiguration {

    override val nalLengthSize: Int
        get() = CodecConfiguration.NO_NAL_LENGTHS

    override val csdBuffers: List<ByteArray>
        get() = listOf(record)

    companion object {
        private const val HEADER_SIZE = 4
        private const val MARKER_AND_VERSION = 0x81
        private const val BYTE_MASK = 0xFF

        fun parse(bytes: ByteArray): Av1ConfigurationRecord? =
            if (bytes.size >= HEADER_SIZE && (bytes[0].toInt() and BYTE_MASK) == MARKER_AND_VERSION) {
                Av1ConfigurationRecord(bytes)
            } else {
                null
            }
    }
}
