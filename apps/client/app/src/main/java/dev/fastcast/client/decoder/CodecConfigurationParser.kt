package dev.fastcast.client.decoder

object CodecConfigurationParser {

    fun parse(mime: String, description: ByteArray): CodecConfiguration? = when (mime) {
        VideoMime.AVC -> AvcConfigurationRecord.parse(description)
        VideoMime.HEVC -> HevcConfigurationRecord.parse(description)
        VideoMime.AV1 -> Av1ConfigurationRecord.parse(description)
        else -> null
    }
}
