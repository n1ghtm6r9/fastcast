package dev.fastcast.client.update

private const val HEX_DIGITS = "0123456789abcdef"
private const val NIBBLE_BITS = 4
private const val NIBBLE_MASK = 0x0F
private const val BYTE_MASK = 0xFF

internal fun ByteArray.toLowerHex(): String {
    val hex = StringBuilder(size * 2)
    for (byte in this) {
        val value = byte.toInt() and BYTE_MASK
        hex.append(HEX_DIGITS[value shr NIBBLE_BITS]).append(HEX_DIGITS[value and NIBBLE_MASK])
    }
    return hex.toString()
}
