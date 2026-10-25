package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import dev.fastcast.client.testing.lengthPrefixed
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertSame
import org.junit.Test

class FourByteNalLengthsAreRewrittenInPlaceTest {

    private val accessUnitDelimiter = hexBytes("09f0")
    private val idrSlice = hexBytes("6588842100a0")
    private val sei = hexBytes("060501ff")
    private val bytes = frameBytes(
        true,
        33L,
        lengthPrefixed(4, accessUnitDelimiter, sei, idrSlice),
    )
    private val frame = requireNotNull(FrameMessage.parse(bytes))
    private val sample = requireNotNull(AnnexBConverter.convert(frame, 4))

    @Test
    fun `the sample reuses the received array`() {
        assertSame(bytes, sample.data)
    }

    @Test
    fun `the sample starts where the payload starts`() {
        assertEquals(FrameMessage.HEADER_SIZE, sample.offset)
        assertEquals(bytes.size - FrameMessage.HEADER_SIZE, sample.size)
    }

    @Test
    fun `every length prefix becomes a start code in front of its nal unit`() {
        val expected = hexBytes("00000001") + accessUnitDelimiter +
            hexBytes("00000001") + sei +
            hexBytes("00000001") + idrSlice

        assertArrayEquals(expected, sample.data.copyOfRange(sample.offset, sample.offset + sample.size))
    }
}
