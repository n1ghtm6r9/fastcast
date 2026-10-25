package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealHevcConfigurationRecord
import org.junit.Assert.assertEquals
import org.junit.Test

class TheHevcNalLengthSizeFollowsTheLowTwoBitsOfByteTwentyOneTest {

    private fun lengthSizeFor(byteTwentyOne: Int): Int {
        val bytes = RealHevcConfigurationRecord.BYTES.copyOf().also { it[21] = byteTwentyOne.toByte() }
        return requireNotNull(HevcConfigurationRecord.parse(bytes)).nalLengthSize
    }

    @Test
    fun `the value is the low two bits plus one`() {
        assertEquals(1, lengthSizeFor(0xFC))
        assertEquals(2, lengthSizeFor(0xFD))
        assertEquals(3, lengthSizeFor(0xFE))
        assertEquals(4, lengthSizeFor(0xFF))
    }

    @Test
    fun `the other bits of the byte do not matter`() {
        assertEquals(4, lengthSizeFor(0x0F))
        assertEquals(1, lengthSizeFor(0x00))
        assertEquals(3, lengthSizeFor(0xAA))
    }
}
