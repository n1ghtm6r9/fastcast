package dev.fastcast.client.decoder

import dev.fastcast.client.testing.anHevcArray
import dev.fastcast.client.testing.anHevcConfigurationRecord
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class AnHevcConfigurationRecordWithoutAllThreeParameterSetsIsRejectedTest {

    private val vps = anHevcArray(32, hexBytes("4001aa"))
    private val sps = anHevcArray(33, hexBytes("4201bb"))
    private val pps = anHevcArray(34, hexBytes("4401cc"))

    @Test
    fun `a record with all three parameter sets is accepted`() {
        assertNotNull(HevcConfigurationRecord.parse(anHevcConfigurationRecord(4, vps, sps, pps)))
    }

    @Test
    fun `a record with no video parameter set is rejected`() {
        assertNull(HevcConfigurationRecord.parse(anHevcConfigurationRecord(4, sps, pps)))
    }

    @Test
    fun `a record with no sequence parameter set is rejected`() {
        assertNull(HevcConfigurationRecord.parse(anHevcConfigurationRecord(4, vps, pps)))
    }

    @Test
    fun `a record with no picture parameter set is rejected`() {
        assertNull(HevcConfigurationRecord.parse(anHevcConfigurationRecord(4, vps, sps)))
    }

    @Test
    fun `a record whose arrays hold no nal units is rejected`() {
        val record = anHevcConfigurationRecord(4, anHevcArray(32), anHevcArray(33), anHevcArray(34))

        assertNull(HevcConfigurationRecord.parse(record))
    }

    @Test
    fun `a record without arrays is rejected`() {
        assertNull(HevcConfigurationRecord.parse(anHevcConfigurationRecord(4)))
    }

    @Test
    fun `a record of other nal unit types only is rejected`() {
        val record = anHevcConfigurationRecord(4, anHevcArray(39, hexBytes("4e0105")))

        assertNull(HevcConfigurationRecord.parse(record))
    }

    @Test
    fun `an empty array is rejected`() {
        assertNull(HevcConfigurationRecord.parse(ByteArray(0)))
    }
}
