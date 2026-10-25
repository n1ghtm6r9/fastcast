package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class TheParameterSetsAreExtractedFromARealAvcConfigurationRecordTest {

    private val startCode = byteArrayOf(0, 0, 0, 1)
    private val record = requireNotNull(AvcConfigurationRecord.parse(RealAvcConfigurationRecord.BYTES))

    @Test
    fun `the nal length size is four`() {
        assertEquals(4, record.nalLengthSize)
    }

    @Test
    fun `the single sequence parameter set is found`() {
        assertEquals(1, record.sequenceParameterSets.size)
        assertArrayEquals(RealAvcConfigurationRecord.SEQUENCE_PARAMETER_SET, record.sequenceParameterSets.single())
    }

    @Test
    fun `the single picture parameter set is found`() {
        assertEquals(1, record.pictureParameterSets.size)
        assertArrayEquals(RealAvcConfigurationRecord.PICTURE_PARAMETER_SET, record.pictureParameterSets.single())
    }

    @Test
    fun `csd0 is the sequence parameter set behind a start code`() {
        assertArrayEquals(startCode + RealAvcConfigurationRecord.SEQUENCE_PARAMETER_SET, record.csd0)
    }

    @Test
    fun `csd1 is the picture parameter set behind a start code`() {
        assertArrayEquals(startCode + RealAvcConfigurationRecord.PICTURE_PARAMETER_SET, record.csd1)
    }

    @Test
    fun `the csd buffers are csd0 followed by csd1`() {
        assertEquals(2, record.csdBuffers.size)
        assertArrayEquals(record.csd0, record.csdBuffers[0])
        assertArrayEquals(record.csd1, record.csdBuffers[1])
    }

    @Test
    fun `the parameter sets carry the h264 nal unit types seven and eight`() {
        assertEquals(7, record.csd0[4].toInt() and 0x1F)
        assertEquals(8, record.csd1[4].toInt() and 0x1F)
    }

    @Test
    fun `the trailing high profile extension bytes are ignored`() {
        val withoutExtension = RealAvcConfigurationRecord.BYTES.copyOf(RealAvcConfigurationRecord.BYTES.size - 4)

        val parsed = requireNotNull(AvcConfigurationRecord.parse(withoutExtension))

        assertArrayEquals(record.csd0, parsed.csd0)
        assertArrayEquals(record.csd1, parsed.csd1)
    }
}
