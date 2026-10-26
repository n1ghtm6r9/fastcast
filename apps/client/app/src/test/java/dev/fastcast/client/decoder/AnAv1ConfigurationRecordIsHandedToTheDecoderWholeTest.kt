package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealAv1ConfigurationRecord
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AnAv1ConfigurationRecordIsHandedToTheDecoderWholeTest {

    @Test
    fun `the whole record is the only codec specific buffer`() {
        val record = requireNotNull(Av1ConfigurationRecord.parse(RealAv1ConfigurationRecord.BYTES))

        assertEquals(1, record.csdBuffers.size)
        assertArrayEquals(RealAv1ConfigurationRecord.BYTES, record.csdBuffers.single())
    }

    @Test
    fun `av1 frames carry no nal lengths`() {
        val record = requireNotNull(Av1ConfigurationRecord.parse(RealAv1ConfigurationRecord.BYTES))

        assertEquals(0, record.nalLengthSize)
    }

    @Test
    fun `a record without the av1c marker is rejected`() {
        val unmarked = RealAv1ConfigurationRecord.BYTES.copyOf().also { it[0] = 0x01 }

        assertNull(Av1ConfigurationRecord.parse(unmarked))
    }

    @Test
    fun `a record cut inside its four byte header is rejected`() {
        assertNull(Av1ConfigurationRecord.parse(hexBytes("810e4c")))
    }
}
