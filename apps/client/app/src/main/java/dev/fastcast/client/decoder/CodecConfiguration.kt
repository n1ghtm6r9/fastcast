package dev.fastcast.client.decoder

interface CodecConfiguration {

    val nalLengthSize: Int

    val csdBuffers: List<ByteArray>
}
