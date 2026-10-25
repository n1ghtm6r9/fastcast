package dev.fastcast.client.decoder

import dev.fastcast.client.testing.anHevcArray
import dev.fastcast.client.testing.anHevcConfigurationRecord
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class TheHevcParameterSetsAreOrderedByKindWhateverTheArrayOrderTest {

    private val startCode = "00000001"
    private val vps = hexBytes("4001aa")
    private val sps = hexBytes("4201bb")
    private val pps = hexBytes("4401cc")

    @Test
    fun `arrays stored in reverse order still yield video then sequence then picture sets`() {
        val bytes = anHevcConfigurationRecord(4, anHevcArray(34, pps), anHevcArray(33, sps), anHevcArray(32, vps))

        val record = requireNotNull(HevcConfigurationRecord.parse(bytes))

        assertArrayEquals(hexBytes(startCode + "4001aa" + startCode + "4201bb" + startCode + "4401cc"), record.csd0)
    }

    @Test
    fun `several nal units in one array keep their order`() {
        val secondSps = hexBytes("4201dd")
        val bytes = anHevcConfigurationRecord(
            4,
            anHevcArray(32, vps),
            anHevcArray(33, sps, secondSps),
            anHevcArray(34, pps),
        )

        val record = requireNotNull(HevcConfigurationRecord.parse(bytes))

        assertEquals(2, record.sequenceParameterSets.size)
        assertArrayEquals(
            hexBytes(startCode + "4001aa" + startCode + "4201bb" + startCode + "4201dd" + startCode + "4401cc"),
            record.csd0,
        )
    }

    @Test
    fun `nal units of one kind spread over several arrays are all collected`() {
        val secondPps = hexBytes("4401ee")
        val bytes = anHevcConfigurationRecord(
            4,
            anHevcArray(34, pps),
            anHevcArray(32, vps),
            anHevcArray(33, sps),
            anHevcArray(34, secondPps),
        )

        val record = requireNotNull(HevcConfigurationRecord.parse(bytes))

        assertEquals(2, record.pictureParameterSets.size)
        assertArrayEquals(
            hexBytes(startCode + "4001aa" + startCode + "4201bb" + startCode + "4401cc" + startCode + "4401ee"),
            record.csd0,
        )
    }

    @Test
    fun `the complete flag of an array does not matter`() {
        val incompleteVps = anHevcArray(32, vps).also { it[0] = (it[0].toInt() and 0x7F).toByte() }
        val bytes = anHevcConfigurationRecord(4, incompleteVps, anHevcArray(33, sps), anHevcArray(34, pps))

        assertEquals(1, requireNotNull(HevcConfigurationRecord.parse(bytes)).videoParameterSets.size)
    }
}
