package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNalLengthSizeFollowsTheLowTwoBitsOfByteFourTest {

    private fun lengthSizeFor(byteFour: Int): Int {
        val bytes = RealAvcConfigurationRecord.BYTES.copyOf().also { it[4] = byteFour.toByte() }
        return requireNotNull(AvcConfigurationRecord.parse(bytes)).nalLengthSize
    }

    @Test
    fun `the value is the low two bits plus one`() {
        assertEquals(1, lengthSizeFor(0xFC))
        assertEquals(2, lengthSizeFor(0xFD))
        assertEquals(3, lengthSizeFor(0xFE))
        assertEquals(4, lengthSizeFor(0xFF))
    }

    @Test
    fun `the reserved upper bits do not matter`() {
        assertEquals(4, lengthSizeFor(0x03))
        assertEquals(2, lengthSizeFor(0x01))
    }
}
