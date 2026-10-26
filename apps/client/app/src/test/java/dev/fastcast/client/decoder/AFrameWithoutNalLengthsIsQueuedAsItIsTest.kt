package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.RealAv1ConfigurationRecord
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Test

class AFrameWithoutNalLengthsIsQueuedAsItIsTest {

    private val temporalUnit =
        hexBytes("1200") + RealAv1ConfigurationRecord.SEQUENCE_HEADER_OBU + hexBytes("32051000112233")
    private val bytes = frameBytes(true, 33L, temporalUnit)
    private val frame = requireNotNull(FrameMessage.parse(bytes))
    private val sample = requireNotNull(AnnexBConverter.convert(frame, 0))

    @Test
    fun `the sample reuses the received array`() {
        assertSame(bytes, sample.data)
    }

    @Test
    fun `the sample is the whole payload`() {
        assertEquals(FrameMessage.HEADER_SIZE, sample.offset)
        assertEquals(temporalUnit.size, sample.size)
    }

    @Test
    fun `the payload is left as it is`() {
        assertArrayEquals(temporalUnit, sample.data.copyOfRange(sample.offset, sample.offset + sample.size))
    }

    @Test
    fun `the sample keeps the timestamp and the key flag`() {
        assertEquals(33_000L, sample.presentationTimeUs)
        assertTrue(sample.keyframe)
    }
}
