package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AFrameWhoseNalLengthsDoNotTileThePayloadIsRejectedTest {

    private fun frameWith(payloadHex: String): FrameMessage =
        requireNotNull(FrameMessage.parse(frameBytes(true, 0L, hexBytes(payloadHex))))

    @Test
    fun `a nal unit longer than the payload is rejected`() {
        assertNull(AnnexBConverter.convert(frameWith("00000064" + "6588"), 4))
        assertNull(AnnexBConverter.convert(frameWith("0064" + "6588"), 2))
    }

    @Test
    fun `a zero length nal unit is rejected`() {
        assertNull(AnnexBConverter.convert(frameWith("00000000" + "6588"), 4))
        assertNull(AnnexBConverter.convert(frameWith("0000" + "6588"), 2))
    }

    @Test
    fun `a payload that ends inside a length prefix is rejected`() {
        assertNull(AnnexBConverter.convert(frameWith("00000002" + "6588" + "000000"), 4))
        assertNull(AnnexBConverter.convert(frameWith("0002" + "6588" + "00"), 2))
    }

    @Test
    fun `a payload shorter than one length prefix is rejected`() {
        assertNull(AnnexBConverter.convert(frameWith("010203"), 4))
    }

    @Test
    fun `a negative looking length with the top bit set is rejected`() {
        assertNull(AnnexBConverter.convert(frameWith("ffffffff" + "6588"), 4))
    }

    @Test
    fun `a rejected frame is left untouched`() {
        val bytes = frameBytes(true, 0L, hexBytes("00000002" + "6588" + "000000"))
        val frame = requireNotNull(FrameMessage.parse(bytes.copyOf()))

        AnnexBConverter.convert(frame, 4)

        assertArrayEquals(bytes, frame.data)
    }
}
