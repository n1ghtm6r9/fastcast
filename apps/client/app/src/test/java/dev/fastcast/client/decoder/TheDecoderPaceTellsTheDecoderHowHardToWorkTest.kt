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
    fun `undeclared names no rate at all`() {
        assertEquals(1, DecoderPace.UNDECLARED.priority)
        assertNull(DecoderPace.UNDECLARED.frameRate(120))
        assertNull(DecoderPace.UNDECLARED.operatingRate(120))
    }
}
