package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDecoderTweakChangesOneThingAboutDecodingTest {

    @Test
    fun `no tweak shows every frame at its own time without asking the tunnel for low latency`() {
        assertFalse(DecoderTweak.NONE.lowLatency)
        assertTrue(DecoderTweak.NONE.rendersOutput)
        assertEquals(8_333L, DecoderTweak.NONE.presentationTimeUs(8_333L))
    }

    @Test
    fun `low latency asks the tunneled decoder for low latency and changes nothing else`() {
        assertTrue(DecoderTweak.LOW_LATENCY.lowLatency)
        assertTrue(DecoderTweak.LOW_LATENCY.rendersOutput)
        assertEquals(8_333L, DecoderTweak.LOW_LATENCY.presentationTimeUs(8_333L))
    }

    @Test
    fun `discarding output decodes every frame without showing it`() {
        assertFalse(DecoderTweak.DISCARD_OUTPUT.lowLatency)
        assertFalse(DecoderTweak.DISCARD_OUTPUT.rendersOutput)
        assertEquals(8_333L, DecoderTweak.DISCARD_OUTPUT.presentationTimeUs(8_333L))
    }

    @Test
    fun `stretched timestamps tell the decoder the frames come at half the rate`() {
        assertFalse(DecoderTweak.STRETCHED_TIMESTAMPS.lowLatency)
        assertTrue(DecoderTweak.STRETCHED_TIMESTAMPS.rendersOutput)
        assertEquals(16_666L, DecoderTweak.STRETCHED_TIMESTAMPS.presentationTimeUs(8_333L))
    }
}
