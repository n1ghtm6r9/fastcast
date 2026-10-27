package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test

class TheUsualDecoderSetupPlaysInRealtimeTest {

    @Test
    fun `the usual stream decodes in realtime without tweaks or a look at vendor parameters`() {
        assertEquals(DecoderPace.REALTIME, DecoderSetup.USUAL.pace)
        assertEquals(DecoderTweak.NONE, DecoderSetup.USUAL.tweak)
        assertFalse(DecoderSetup.USUAL.inspectsVendorParameters)
    }

    @Test
    fun `the usual tunnel starts its audio with the decoder and shows each frame 50 ms after the audio clock`() {
        assertEquals(AudioStart.WITH_DECODER, DecoderSetup.USUAL.timing.audioStart)
        assertEquals(50_000L, DecoderSetup.USUAL.timing.videoDelayUs)
        assertEquals(TunnelClock.VIDEO_DELAY_US, TunnelTiming.USUAL.videoDelayUs)
    }
}
