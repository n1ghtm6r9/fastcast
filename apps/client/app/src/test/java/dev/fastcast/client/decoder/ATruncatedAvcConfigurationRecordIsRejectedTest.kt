package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class ATruncatedAvcConfigurationRecordIsRejectedTest {

    private val endOfPictureParameterSet = 43

    @Test
    fun `every prefix that cuts a parameter set short is rejected`() {
        for (size in 0 until endOfPictureParameterSet) {
            val truncated = RealAvcConfigurationRecord.BYTES.copyOf(size)

            assertNull("size $size", AvcConfigurationRecord.parse(truncated))
        }
    }

    @Test
    fun `a record that ends right after the picture parameter set is complete`() {
        val trimmed = RealAvcConfigurationRecord.BYTES.copyOf(endOfPictureParameterSet)

        assertNotNull(AvcConfigurationRecord.parse(trimmed))
    }

    @Test
    fun `a parameter set longer than the remaining bytes is rejected`() {
        val bytes = RealAvcConfigurationRecord.BYTES.copyOf().also {
            it[6] = 0xFF.toByte()
            it[7] = 0xFF.toByte()
        }

        assertNull(AvcConfigurationRecord.parse(bytes))
    }
}
