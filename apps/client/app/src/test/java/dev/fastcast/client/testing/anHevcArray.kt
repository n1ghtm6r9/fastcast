package dev.fastcast.client.testing

private const val COMPLETE_ARRAY_FLAG = 0x80

fun anHevcArray(type: Int, vararg nalUnits: ByteArray): ByteArray {
    val typeByte = (COMPLETE_ARRAY_FLAG or type).toByte()
    val header = byteArrayOf(typeByte, (nalUnits.size shr Byte.SIZE_BITS).toByte(), nalUnits.size.toByte())
    return nalUnits.fold(header) { array, nalUnit ->
        array + byteArrayOf((nalUnit.size shr Byte.SIZE_BITS).toByte(), nalUnit.size.toByte()) + nalUnit
    }
}
