package dev.fastcast.client.decoder

interface CodecConfiguration {

    val nalLengthSize: Int

    val csdBuffers: List<ByteArray>

    companion object {
        const val NO_NAL_LENGTHS = 0
    }
}
