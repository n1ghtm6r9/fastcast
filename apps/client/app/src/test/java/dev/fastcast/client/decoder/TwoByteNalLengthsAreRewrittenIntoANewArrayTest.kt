package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import dev.fastcast.client.testing.lengthPrefixed
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNotSame
import org.junit.Test

class TwoByteNalLengthsAreRewrittenIntoANewArrayTest {

    private val accessUnitDelimiter = hexBytes("09f0")
    private val idrSlice = hexBytes("6588842100a0")
    private val sei = hexBytes("060501ff")
    private val bytes = frameBytes(
        false,
        50L,
        lengthPrefixed(2, accessUnitDelimiter, sei, idrSlice),
    )
    private val frame = requireNotNull(FrameMessage.parse(bytes))
    private val sample = requireNotNull(AnnexBConverter.convert(frame, 2))

    @Test
    fun `the sample lives in a fresh array`() {
        assertNotSame(bytes, sample.data)
    }

    @Test
    fun `the sample starts at the beginning of the new array`() {
        assertEquals(0, sample.offset)
        assertEquals(sample.data.size, sample.size)
    }

    @Test
    fun `the sample grows by two bytes for every nal unit`() {
        assertEquals(bytes.size - FrameMessage.HEADER_SIZE + 3 * 2, sample.size)
    }

    @Test
    fun `every length prefix becomes a start code in front of its nal unit`() {
        val expected = hexBytes("00000001") + accessUnitDelimiter +
            hexBytes("00000001") + sei +
            hexBytes("00000001") + idrSlice

        assertArrayEquals(expected, sample.data)
    }

    @Test
    fun `the received frame is left as it arrived`() {
        val original = frameBytes(false, 50L, lengthPrefixed(2, accessUnitDelimiter, sei, idrSlice))

        assertArrayEquals(original, bytes)
    }
}
