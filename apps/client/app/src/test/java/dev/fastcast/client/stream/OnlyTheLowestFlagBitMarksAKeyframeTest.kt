package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Test

class OnlyTheLowestFlagBitMarksAKeyframeTest {

    @Test
    fun `a frame is a keyframe exactly when bit zero of the flags is set`() {
        val expectations = mapOf(
            0x00 to false,
            0x01 to true,
            0x02 to false,
            0x03 to true,
            0x80 to false,
            0xFE to false,
            0xFF to true,
        )

        expectations.forEach { (flags, expected) ->
            val frame = requireNotNull(FrameMessage.parse(byteArrayOf(flags.toByte(), 0, 0, 0, 0, 7)))

            assertEquals("flags 0x${flags.toString(16)}", expected, frame.keyframe)
        }
    }
}
