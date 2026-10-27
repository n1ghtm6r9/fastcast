package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheDecoderPaceTellsTheDecoderHowHardToWorkTest {

    @Test
    fun `realtime promises the stream rate at realtime priority`() {
        assertEquals(0, DecoderPace.REALTIME.priority)
        assertEquals(120, DecoderPace.REALTIME.frameRate(120))
        assertEquals(120, DecoderPace.REALTIME.operatingRate(120))
    }

    @Test
    fun `best effort names the stream rate without the realtime promise`() {
        assertEquals(1, DecoderPace.BEST_EFFORT.priority)
        assertEquals(120, DecoderPace.BEST_EFFORT.frameRate(120))
        assertEquals(120, DecoderPace.BEST_EFFORT.operatingRate(120))
    }

    @Test
    fun `full speed asks for the highest operating rate a codec reads safely`() {
        assertEquals(1, DecoderPace.FULL_SPEED.priority)
        assertEquals(120, DecoderPace.FULL_SPEED.frameRate(120))
        assertEquals(32_767, DecoderPace.FULL_SPEED.operatingRate(120))
    }

    @Test
    fun `undeclared names no rate at all`() {
        assertEquals(1, DecoderPace.UNDECLARED.priority)
        assertNull(DecoderPace.UNDECLARED.frameRate(120))
        assertNull(DecoderPace.UNDECLARED.operatingRate(120))
    }

    @Test
    fun `a best effort mode runs the decoder at full speed and a promised one in realtime`() {
        assertEquals(DecoderPace.FULL_SPEED, DecoderPace.of(bestEffort = true))
        assertEquals(DecoderPace.REALTIME, DecoderPace.of(bestEffort = false))
    }
}
