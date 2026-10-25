package dev.fastcast.client.stream

import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class AFrameWithoutAPayloadIsRejectedTest {

    @Test
    fun `anything up to the bare header is rejected`() {
        for (size in 0..FrameMessage.HEADER_SIZE) {
            assertNull("size $size", FrameMessage.parse(ByteArray(size)))
        }
    }

    @Test
    fun `a single payload byte is enough`() {
        assertNotNull(FrameMessage.parse(ByteArray(FrameMessage.HEADER_SIZE + 1)))
    }
}
