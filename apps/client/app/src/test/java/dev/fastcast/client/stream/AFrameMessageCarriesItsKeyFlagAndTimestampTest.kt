package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class AFrameMessageCarriesItsKeyFlagAndTimestampTest {

    @Test
    fun `a keyframe reports the key flag and its timestamp`() {
        val frame = requireNotNull(FrameMessage.parse(frameBytes(true, 1_000L, byteArrayOf(9, 9))))

        assertTrue(frame.keyframe)
        assertEquals(1_000L, frame.timestampMs)
    }

    @Test
    fun `a delta frame reports no key flag and its timestamp`() {
        val frame = requireNotNull(FrameMessage.parse(frameBytes(false, 16L, byteArrayOf(9, 9))))

        assertFalse(frame.keyframe)
        assertEquals(16L, frame.timestampMs)
    }
}
