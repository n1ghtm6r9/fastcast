package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test

class TheUsualDecoderSetupPlaysInRealtimeTest {

    @Test
    fun `the usual stream decodes in realtime at low latency without a look at vendor parameters`() {
        assertEquals(DecoderPace.REALTIME, DecoderSetup.USUAL.pace)
        assertEquals(DecoderTweak.LOW_LATENCY, DecoderSetup.USUAL.tweak)
        assertFalse(DecoderSetup.USUAL.inspectsVendorParameters)
    }

    @Test
    fun `the usual stream shows every frame it decodes`() {
        assertEquals(0, DecoderSetup.USUAL.decodeOnlyFrames)
    }

    @Test
    fun `the usual tunnel starts its video on the running audio clock and shows each frame 8 ms after it`() {
        assertEquals(VideoStart.ON_RUNNING_CLOCK, DecoderSetup.USUAL.timing.videoStart)
        assertEquals(8_000L, DecoderSetup.USUAL.timing.videoDelayUs)
        assertEquals(TunnelClock.VIDEO_DELAY_US, TunnelTiming.USUAL.videoDelayUs)
    }
}
