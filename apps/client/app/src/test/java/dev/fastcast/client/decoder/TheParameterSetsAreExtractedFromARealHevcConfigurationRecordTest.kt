package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealHevcConfigurationRecord
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class TheParameterSetsAreExtractedFromARealHevcConfigurationRecordTest {

    private val startCode = byteArrayOf(0, 0, 0, 1)
    private val record = requireNotNull(HevcConfigurationRecord.parse(RealHevcConfigurationRecord.BYTES))

    private fun nalUnitType(nalUnit: ByteArray): Int = (nalUnit[0].toInt() shr 1) and 0x3F

    @Test
    fun `the nal length size is four`() {
        assertEquals(4, record.nalLengthSize)
    }

    @Test
    fun `the single video parameter set is found`() {
        assertEquals(1, record.videoParameterSets.size)
        assertArrayEquals(RealHevcConfigurationRecord.VIDEO_PARAMETER_SET, record.videoParameterSets.single())
    }

    @Test
    fun `the single sequence parameter set is found`() {
        assertEquals(1, record.sequenceParameterSets.size)
        assertArrayEquals(RealHevcConfigurationRecord.SEQUENCE_PARAMETER_SET, record.sequenceParameterSets.single())
    }

    @Test
    fun `the single picture parameter set is found`() {
        assertEquals(1, record.pictureParameterSets.size)
        assertArrayEquals(RealHevcConfigurationRecord.PICTURE_PARAMETER_SET, record.pictureParameterSets.single())
    }

    @Test
    fun `csd0 is the video then sequence then picture parameter set behind start codes`() {
        val expected = startCode + RealHevcConfigurationRecord.VIDEO_PARAMETER_SET +
            startCode + RealHevcConfigurationRecord.SEQUENCE_PARAMETER_SET +
            startCode + RealHevcConfigurationRecord.PICTURE_PARAMETER_SET

        assertArrayEquals(expected, record.csd0)
    }

    @Test
    fun `the parameter sets carry the h265 nal unit types thirty two thirty three and thirty four`() {
        assertEquals(32, nalUnitType(record.videoParameterSets.single()))
        assertEquals(33, nalUnitType(record.sequenceParameterSets.single()))
        assertEquals(34, nalUnitType(record.pictureParameterSets.single()))
    }

    @Test
    fun `csd0 is the only csd buffer`() {
        assertEquals(1, record.csdBuffers.size)
        assertArrayEquals(record.csd0, record.csdBuffers.single())
    }

    @Test
    fun `bytes after the last array are ignored`() {
        val withTrailingBytes = RealHevcConfigurationRecord.BYTES + byteArrayOf(0x55, 0x66, 0x77)

        val parsed = requireNotNull(HevcConfigurationRecord.parse(withTrailingBytes))

        assertArrayEquals(record.csd0, parsed.csd0)
    }
}
