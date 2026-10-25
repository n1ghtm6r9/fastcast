package dev.fastcast.client.decoder

import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.testing.frameBytes
import dev.fastcast.client.testing.hexBytes
import dev.fastcast.client.testing.lengthPrefixed
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ASampleCarriesTheTimestampInMicrosecondsAndTheKeyFlagTest {

    private fun sampleOf(keyframe: Boolean, timestampMs: Long, lengthSize: Int): VideoSample {
        val payload = lengthPrefixed(lengthSize, hexBytes("6588"))
        val frame = requireNotNull(FrameMessage.parse(frameBytes(keyframe, timestampMs, payload)))
        return requireNotNull(AnnexBConverter.convert(frame, lengthSize))
    }

    @Test
    fun `the presentation time is the timestamp times a thousand`() {
        assertEquals(1_500_000L, sampleOf(false, 1_500L, 4).presentationTimeUs)
        assertEquals(1_500_000L, sampleOf(false, 1_500L, 2).presentationTimeUs)
    }

    @Test
    fun `a keyframe stays a keyframe`() {
        assertTrue(sampleOf(true, 0L, 4).keyframe)
        assertTrue(sampleOf(true, 0L, 2).keyframe)
    }

    @Test
    fun `a delta frame stays a delta frame`() {
        assertFalse(sampleOf(false, 0L, 4).keyframe)
        assertFalse(sampleOf(false, 0L, 2).keyframe)
    }
}
