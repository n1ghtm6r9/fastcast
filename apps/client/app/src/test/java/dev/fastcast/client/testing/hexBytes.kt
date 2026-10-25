package dev.fastcast.client.testing

fun hexBytes(hex: String): ByteArray =
    ByteArray(hex.length / 2) { index -> hex.substring(index * 2, index * 2 + 2).toInt(16).toByte() }
