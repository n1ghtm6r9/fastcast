package dev.fastcast.client.testing

fun lengthPrefixed(lengthSize: Int, vararg nalUnits: ByteArray): ByteArray =
    nalUnits.fold(ByteArray(0)) { accumulated, nalUnit ->
        val prefix = ByteArray(lengthSize) { index ->
            (nalUnit.size shr (Byte.SIZE_BITS * (lengthSize - 1 - index))).toByte()
        }
        accumulated + prefix + nalUnit
    }
