package dev.fastcast.client.decoder

object CodecConfigurationParser {

    fun parse(mime: String, description: ByteArray): CodecConfiguration? = when (mime) {
        VideoMime.AVC -> AvcConfigurationRecord.parse(description)
        VideoMime.HEVC -> HevcConfigurationRecord.parse(description)
        else -> null
    }
}
