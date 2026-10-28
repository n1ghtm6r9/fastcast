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
        assertEquals(emptyMap<String, Number>(), DecoderTweak.NONE.vendorValues())
        assertEquals(emptyMap<String, Number>(), DecoderTweak.NONE.liveVendorValues())
    }

    @Test
    fun `the low latency tweak asks the tunnel for low latency`() {
        assertTrue(DecoderTweak.LOW_LATENCY.lowLatency)
        assertTrue(DecoderTweak.LOW_LATENCY.usesStandardLowLatency)
        assertNull(DecoderTweak.LOW_LATENCY.operatingRate(120))
        assertEquals(emptyMap<String, Number>(), DecoderTweak.LOW_LATENCY.vendorValues())
        assertEquals(emptyMap<String, Number>(), DecoderTweak.LOW_LATENCY.liveVendorValues())
    }

    @Test
    fun `playing as a game swaps the standard hint for the MediaTek game and low latency modes at the stream rate`() {
        assertFalse(DecoderTweak.MEDIATEK_GAME.lowLatency)
        assertFalse(DecoderTweak.MEDIATEK_GAME.usesStandardLowLatency)
        assertEquals(120, DecoderTweak.MEDIATEK_GAME.operatingRate(120))
        assertEquals(GAME, DecoderTweak.MEDIATEK_GAME.vendorValues())
        assertEquals(emptyMap<String, Number>(), DecoderTweak.MEDIATEK_GAME.liveVendorValues())
    }

    @Test
    fun `playing as a game live asks for the game modes again once the decoder runs`() {
        assertFalse(DecoderTweak.MEDIATEK_GAME_LIVE.lowLatency)
        assertFalse(DecoderTweak.MEDIATEK_GAME_LIVE.usesStandardLowLatency)
        assertEquals(120, DecoderTweak.MEDIATEK_GAME_LIVE.operatingRate(120))
        assertEquals(GAME, DecoderTweak.MEDIATEK_GAME_LIVE.vendorValues())
        assertEquals(GAME, DecoderTweak.MEDIATEK_GAME_LIVE.liveVendorValues())
    }

    @Test
    fun `low latency as a game live keeps the standard hint and asks for the game modes once the decoder runs`() {
        assertTrue(DecoderTweak.LOW_LATENCY_GAME_LIVE.lowLatency)
        assertTrue(DecoderTweak.LOW_LATENCY_GAME_LIVE.usesStandardLowLatency)
        assertNull(DecoderTweak.LOW_LATENCY_GAME_LIVE.operatingRate(120))
        assertEquals(emptyMap<String, Number>(), DecoderTweak.LOW_LATENCY_GAME_LIVE.vendorValues())
        assertEquals(GAME, DecoderTweak.LOW_LATENCY_GAME_LIVE.liveVendorValues())
    }

    @Test
    fun `forcing real time asks the MediaTek decoder for it before and after it starts`() {
        assertFalse(DecoderTweak.MEDIATEK_REAL_TIME.lowLatency)
        assertTrue(DecoderTweak.MEDIATEK_REAL_TIME.usesStandardLowLatency)
        assertNull(DecoderTweak.MEDIATEK_REAL_TIME.operatingRate(120))
        assertEquals(REAL_TIME, DecoderTweak.MEDIATEK_REAL_TIME.vendorValues())
        assertEquals(REAL_TIME, DecoderTweak.MEDIATEK_REAL_TIME.liveVendorValues())
    }

    private companion object {
        val GAME = mapOf("vendor.mtk-codec2.game-mode" to 1, "vendor.mtk-codec2.low-latency-mode" to 1)
        val REAL_TIME = mapOf("vendor.mtk-iview.force-real-time" to 1)
    }
}
