package dev.fastcast.client.testing

import dev.fastcast.client.stream.FrameMessage

fun aFrameMessage(
    keyframe: Boolean = false,
    timestampMs: Long = 0L,
    payload: ByteArray = byteArrayOf(1),
): FrameMessage = requireNotNull(FrameMessage.parse(frameBytes(keyframe, timestampMs, payload)))
