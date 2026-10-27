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
}
