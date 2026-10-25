package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Test

class AFrameTimestampIsReadLittleEndianAndUnsignedTest {

    @Test
    fun `the four timestamp bytes are little endian`() {
        val frame = requireNotNull(FrameMessage.parse(byteArrayOf(0, 0x01, 0x02, 0x03, 0x04, 7)))

        assertEquals(0x04030201L, frame.timestampMs)
    }

    @Test
    fun `a thousand milliseconds is stored as e8 03 00 00`() {
        val frame = requireNotNull(FrameMessage.parse(byteArrayOf(0, 0xE8.toByte(), 0x03, 0, 0, 7)))

        assertEquals(1_000L, frame.timestampMs)
    }

    @Test
    fun `a timestamp with the top bit set stays positive`() {
        val frame = requireNotNull(FrameMessage.parse(byteArrayOf(0, -1, -1, -1, -1, 7)))

        assertEquals(4_294_967_295L, frame.timestampMs)
    }

    @Test
    fun `the presentation time is the timestamp in microseconds`() {
        val frame = requireNotNull(FrameMessage.parse(byteArrayOf(0, -1, -1, -1, -1, 7)))

        assertEquals(4_294_967_295_000L, frame.presentationTimeUs)
    }
}
