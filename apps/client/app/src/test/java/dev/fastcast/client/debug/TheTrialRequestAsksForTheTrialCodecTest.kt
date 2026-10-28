package dev.fastcast.client.debug

import dev.fastcast.client.decoder.DecoderSetup
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheTrialRequestAsksForTheTrialCodecTest {

    @Test
    fun `a 10-bit trial asks for its size, rate and codec in 10 bits`() {
        val request = TrialRequest.of(trial("hevc", tenBit = true))

        assertEquals(listOf(3840, 2160, 60), listOf(request.width, request.height, request.fps))
        assertEquals(listOf("hevc"), request.codecs)
        assertEquals(setOf("hevc"), request.tenBitCodecs)
    }

    @Test
    fun `an 8-bit trial asks for its codec alone`() {
        val request = TrialRequest.of(trial("avc", tenBit = false))

        assertEquals(listOf("avc"), request.codecs)
        assertEquals(emptySet<String>(), request.tenBitCodecs)
    }

    @Test
    fun `a noise trial asks the server for noise instead of the screen`() {
        assertTrue(TrialRequest.of(trial("hevc", tenBit = true, noise = true)).noise)
        assertFalse(TrialRequest.of(trial("hevc", tenBit = true, noise = false)).noise)
    }

    private fun trial(codec: String, tenBit: Boolean, noise: Boolean = false): StreamTrial = StreamTrial(
        "trial", 3840, 2160, 60, codec, tenBit, tunneled = false, DecoderSetup.USUAL, showsText = false,
        DebugStep.SURFACE, noise,
    )
}
