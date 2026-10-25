package dev.fastcast.client.decoder

import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertNull
import org.junit.Test

class AnAvcConfigurationRecordWithoutParameterSetsIsRejectedTest {

    @Test
    fun `a record with no sequence parameter set is rejected`() {
        val record = hexBytes("0164002affe0" + "01" + "0004" + "68ce0fc8")

        assertNull(AvcConfigurationRecord.parse(record))
    }

    @Test
    fun `a record with no picture parameter set is rejected`() {
        val record = hexBytes("0164002affe1" + "0004" + "6764002a" + "00")

        assertNull(AvcConfigurationRecord.parse(record))
    }

    @Test
    fun `a record with neither is rejected`() {
        assertNull(AvcConfigurationRecord.parse(hexBytes("0164002affe000")))
    }

    @Test
    fun `an empty array is rejected`() {
        assertNull(AvcConfigurationRecord.parse(ByteArray(0)))
    }
}
