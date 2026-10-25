package dev.fastcast.client.testing

private const val LENGTH_SIZE_POSITION = 21
private const val RESERVED_BITS = 0xFC

fun anHevcConfigurationRecord(lengthSize: Int, vararg arrays: ByteArray): ByteArray {
    val header = ByteArray(LENGTH_SIZE_POSITION).also { it[0] = 1 }
    val lengthSizeByte = (RESERVED_BITS or (lengthSize - 1)).toByte()
    return arrays.fold(header + byteArrayOf(lengthSizeByte, arrays.size.toByte())) { record, array -> record + array }
}
