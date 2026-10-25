package dev.fastcast.client.decoder

import dev.fastcast.client.testing.anHevcArray
import dev.fastcast.client.testing.anHevcConfigurationRecord
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Test

class OtherHevcNalUnitTypesAreSkippedTest {

    private val vps = hexBytes("4001aa")
    private val sps = hexBytes("4201bb")
    private val pps = hexBytes("4401cc")
    private val expected = hexBytes("00000001" + "4001aa" + "00000001" + "4201bb" + "00000001" + "4401cc")

    @Test
    fun `sei arrays between the parameter sets are not part of csd0`() {
        val bytes = anHevcConfigurationRecord(
            4,
            anHevcArray(32, vps),
            anHevcArray(39, hexBytes("4e0105"), hexBytes("4e0106")),
            anHevcArray(33, sps),
            anHevcArray(40, hexBytes("5001ff")),
            anHevcArray(34, pps),
        )

        assertArrayEquals(expected, requireNotNull(HevcConfigurationRecord.parse(bytes)).csd0)
    }

    @Test
    fun `an array without nal units is skipped`() {
        val bytes = anHevcConfigurationRecord(
            4,
            anHevcArray(35),
            anHevcArray(32, vps),
            anHevcArray(33, sps),
            anHevcArray(34, pps),
        )

        assertArrayEquals(expected, requireNotNull(HevcConfigurationRecord.parse(bytes)).csd0)
    }

    @Test
    fun `the reserved bit above the nal unit type is ignored`() {
        val reservedVps = anHevcArray(32, vps).also { it[0] = (it[0].toInt() or 0x40).toByte() }
        val bytes = anHevcConfigurationRecord(4, reservedVps, anHevcArray(33, sps), anHevcArray(34, pps))

        assertArrayEquals(expected, requireNotNull(HevcConfigurationRecord.parse(bytes)).csd0)
    }
}
