package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertNull
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDecoderTweakChangesOneThingAboutDecodingTest {

    @Test
    fun `no tweak keeps the standard low latency hint and asks the decoder for nothing more`() {
        assertFalse(DecoderTweak.NONE.lowLatency)
        assertTrue(DecoderTweak.NONE.usesStandardLowLatency)
        assertNull(DecoderTweak.NONE.operatingRate(120))
        assertEquals(emptyMap<String, Number>(), DecoderTweak.NONE.vendorValues(120))
    }

    @Test
    fun `the low latency tweak asks the tunnel for low latency`() {
        assertTrue(DecoderTweak.LOW_LATENCY.lowLatency)
        assertTrue(DecoderTweak.LOW_LATENCY.usesStandardLowLatency)
        assertNull(DecoderTweak.LOW_LATENCY.operatingRate(120))
        assertEquals(emptyMap<String, Number>(), DecoderTweak.LOW_LATENCY.vendorValues(120))
    }

    @Test
    fun `playing as a game swaps the standard hint for the MediaTek game and low latency modes at the stream rate`() {
        assertFalse(DecoderTweak.MEDIATEK_GAME.lowLatency)
        assertFalse(DecoderTweak.MEDIATEK_GAME.usesStandardLowLatency)
        assertEquals(120, DecoderTweak.MEDIATEK_GAME.operatingRate(120))
        assertEquals(
            mapOf("vendor.mtk-codec2.game-mode" to 1, "vendor.mtk-codec2.low-latency-mode" to 1),
            DecoderTweak.MEDIATEK_GAME.vendorValues(120),
        )
    }

    @Test
    fun `forcing the display rate tells the MediaTek decoder the stream runs at 60`() {
        assertTrue(DecoderTweak.FORCED_DISPLAY_RATE.usesStandardLowLatency)
        assertNull(DecoderTweak.FORCED_DISPLAY_RATE.operatingRate(120))
        assertEquals(
            mapOf("vendor.mtk-codec2.force-frame-rate" to 60),
            DecoderTweak.FORCED_DISPLAY_RATE.vendorValues(120),
        )
    }

    @Test
    fun `forcing the stream rate tells the MediaTek decoder the rate the stream really has`() {
        assertTrue(DecoderTweak.FORCED_STREAM_RATE.usesStandardLowLatency)
        assertEquals(
            mapOf("vendor.mtk-codec2.force-frame-rate" to 120),
            DecoderTweak.FORCED_STREAM_RATE.vendorValues(120),
        )
    }

    @Test
    fun `raising the rate limit lets the MediaTek decoder go up to the stream rate`() {
        assertTrue(DecoderTweak.RAISED_RATE_LIMIT.usesStandardLowLatency)
        assertEquals(
            mapOf("vendor.mtk-codec2.max-frame-rate" to 120f),
            DecoderTweak.RAISED_RATE_LIMIT.vendorValues(120),
        )
    }
}
