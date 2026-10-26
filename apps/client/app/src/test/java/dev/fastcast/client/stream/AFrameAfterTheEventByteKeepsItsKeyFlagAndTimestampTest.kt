package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Test

class AFrameAfterTheEventByteKeepsItsKeyFlagAndTimestampTest {

    private val payload = byteArrayOf(10, 11, 12)
    private val event = byteArrayOf(0) + frameBytes(true, 258L, payload)
    private val frame = requireNotNull(FrameMessage.parse(event, 1))

    @Test
    fun `the key flag is read after the event byte`() {
        assertTrue(frame.keyframe)
    }

    @Test
    fun `the timestamp is read after the event byte`() {
        assertEquals(258L, frame.timestampMs)
    }

    @Test
    fun `the payload starts after the event byte and the header`() {
        assertEquals(6, frame.payloadOffset)
        assertEquals(payload.size, frame.payloadSize)
        assertArrayEquals(payload, frame.data.copyOfRange(frame.payloadOffset, frame.payloadOffset + frame.payloadSize))
    }

    @Test
    fun `the frame keeps the received array without copying it`() {
        assertSame(event, frame.data)
    }

    @Test
    fun `an event with nothing after the header is rejected`() {
        assertNull(FrameMessage.parse(ByteArray(1 + FrameMessage.HEADER_SIZE), 1))
    }

    @Test
    fun `a single payload byte after the header is enough`() {
        assertNotNull(FrameMessage.parse(ByteArray(1 + FrameMessage.HEADER_SIZE + 1), 1))
    }

    @Test
    fun `a frame without an offset is read from the start`() {
        val plain = requireNotNull(FrameMessage.parse(frameBytes(true, 258L, payload)))

        assertEquals(FrameMessage.HEADER_SIZE, plain.payloadOffset)
        assertEquals(258L, plain.timestampMs)
    }
}
