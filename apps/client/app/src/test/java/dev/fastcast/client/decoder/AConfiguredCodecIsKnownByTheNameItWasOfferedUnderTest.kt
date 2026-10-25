package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class AConfiguredCodecIsKnownByTheNameItWasOfferedUnderTest {

    @Test
    fun `an hevc codec string is known as hevc`() {
        assertEquals("hevc", SupportedCodecs.nameFor("hvc1.1.6.L153.90"))
    }

    @Test
    fun `an avc codec string is known as avc`() {
        assertEquals("avc", SupportedCodecs.nameFor("avc1.640033"))
    }

    @Test
    fun `a codec that was never offered has no name`() {
        assertNull(SupportedCodecs.nameFor("vp09.00.10.08"))
    }
}
