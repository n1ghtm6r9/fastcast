package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import dev.fastcast.client.testing.lengthPrefixed
import org.junit.Assert.assertArrayEquals
import org.junit.Test

class EveryNalLengthSizeYieldsTheSameAnnexBStreamTest {

    private val nalUnits = arrayOf(hexBytes("09f0"), hexBytes("0605ff"), hexBytes("6588842100a0b1c2d3"))
    private val expected = nalUnits.fold(ByteArray(0)) { annexB, nal -> annexB + hexBytes("00000001") + nal }

    private fun convertedWith(lengthSize: Int): ByteArray {
        val frame = requireNotNull(
            FrameMessage.parse(frameBytes(true, 0L, lengthPrefixed(lengthSize, *nalUnits))),
        )
        val sample = requireNotNull(AnnexBConverter.convert(frame, lengthSize))
        return sample.data.copyOfRange(sample.offset, sample.offset + sample.size)
    }

    @Test
    fun `one byte lengths`() {
        assertArrayEquals(expected, convertedWith(1))
    }

    @Test
    fun `two byte lengths`() {
        assertArrayEquals(expected, convertedWith(2))
    }

    @Test
    fun `three byte lengths`() {
        assertArrayEquals(expected, convertedWith(3))
    }

    @Test
    fun `four byte lengths`() {
        assertArrayEquals(expected, convertedWith(4))
    }
}
