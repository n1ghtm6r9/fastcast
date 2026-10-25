package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealHevcConfigurationRecord
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class ATruncatedHevcConfigurationRecordIsRejectedTest {

    private val endOfPictureParameterSet = 128
    private val pictureParameterSetLengthPosition = 119

    @Test
    fun `the real record ends with its picture parameter set`() {
        assertNotNull(HevcConfigurationRecord.parse(RealHevcConfigurationRecord.BYTES.copyOf(endOfPictureParameterSet)))
    }

    @Test
    fun `every shorter prefix is rejected`() {
        for (size in 0 until endOfPictureParameterSet) {
            val truncated = RealHevcConfigurationRecord.BYTES.copyOf(size)

            assertNull("size $size", HevcConfigurationRecord.parse(truncated))
        }
    }

    @Test
    fun `a nal unit longer than the remaining bytes is rejected`() {
        val bytes = RealHevcConfigurationRecord.BYTES.copyOf().also {
            it[pictureParameterSetLengthPosition] = 0xFF.toByte()
            it[pictureParameterSetLengthPosition + 1] = 0xFF.toByte()
        }

        assertNull(HevcConfigurationRecord.parse(bytes))
    }

    @Test
    fun `an array count larger than the arrays present is rejected`() {
        val bytes = RealHevcConfigurationRecord.BYTES.copyOf().also { it[22] = 0xFF.toByte() }

        assertNull(HevcConfigurationRecord.parse(bytes))
    }
}
