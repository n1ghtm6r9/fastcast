package dev.fastcast.client.decoder

import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class SeveralParameterSetsAreConcatenatedBehindStartCodesTest {

    private val record = requireNotNull(
        AvcConfigurationRecord.parse(
            hexBytes("0164002affe2" + "0003" + "674200" + "0002" + "6743" + "02" + "0002" + "68ce" + "0003" + "68cf01"),
        ),
    )

    @Test
    fun `both sequence parameter sets are listed`() {
        assertEquals(2, record.sequenceParameterSets.size)
        assertEquals(2, record.pictureParameterSets.size)
    }

    @Test
    fun `csd0 holds each sequence parameter set behind its own start code`() {
        assertArrayEquals(hexBytes("00000001" + "674200" + "00000001" + "6743"), record.csd0)
    }

    @Test
    fun `csd1 holds each picture parameter set behind its own start code`() {
        assertArrayEquals(hexBytes("00000001" + "68ce" + "00000001" + "68cf01"), record.csd1)
    }
}
