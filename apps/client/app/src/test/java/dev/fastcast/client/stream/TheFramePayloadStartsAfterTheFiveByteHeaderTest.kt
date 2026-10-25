package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.Assert.assertEquals
import org.junit.Assert.assertSame
import org.junit.Test

class TheFramePayloadStartsAfterTheFiveByteHeaderTest {

    private val bytes = frameBytes(true, 42L, byteArrayOf(10, 11, 12))
    private val frame = requireNotNull(FrameMessage.parse(bytes))

    @Test
    fun `the payload offset is the header size`() {
        assertEquals(5, frame.payloadOffset)
        assertEquals(FrameMessage.HEADER_SIZE, frame.payloadOffset)
    }

    @Test
    fun `the payload size is everything after the header`() {
        assertEquals(3, frame.payloadSize)
    }

    @Test
    fun `the first byte at the payload offset is the first payload byte`() {
        assertEquals(10.toByte(), frame.data[frame.payloadOffset])
    }

    @Test
    fun `the frame keeps the received array without copying it`() {
        assertSame(bytes, frame.data)
    }
}
