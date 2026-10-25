package dev.fastcast.client.decoder

private val START_CODE = byteArrayOf(0, 0, 0, 1)

internal fun List<ByteArray>.withStartCodes(): ByteArray =
    fold(ByteArray(0)) { accumulated, nalUnit -> accumulated + START_CODE + nalUnit }
