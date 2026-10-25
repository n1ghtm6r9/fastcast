package dev.fastcast.client.testing

fun frameBytes(keyframe: Boolean, timestampMs: Long, payload: ByteArray): ByteArray {
    val flags: Byte = if (keyframe) 1 else 0
    val header = byteArrayOf(
        flags,
        timestampMs.toByte(),
        (timestampMs shr 8).toByte(),
        (timestampMs shr 16).toByte(),
        (timestampMs shr 24).toByte(),
    )
    return header + payload
}
