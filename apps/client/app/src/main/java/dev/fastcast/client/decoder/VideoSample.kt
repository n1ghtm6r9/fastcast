package dev.fastcast.client.decoder

class VideoSample(
    val data: ByteArray,
    val offset: Int,
    val size: Int,
    val presentationTimeUs: Long,
    val keyframe: Boolean,
)
