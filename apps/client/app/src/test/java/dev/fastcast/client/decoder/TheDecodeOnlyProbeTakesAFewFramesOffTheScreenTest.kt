package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDecodeOnlyProbeTakesAFewFramesOffTheScreenTest {

    @Test
    fun `a probe for no frames shows every frame`() {
        val probe = DecodeOnlyProbe(frames = 0)

        assertEquals(0, (0..20).count { probe.decodesOnly(it * HALF_SECOND) })
    }

    @Test
    fun `the first frame two seconds after the probe starts is only decoded`() {
        val probe = DecodeOnlyProbe(frames = 5)

        assertFalse(probe.decodesOnly(START))
        assertFalse(probe.decodesOnly(START + 2 * SECOND - 1))
        assertTrue(probe.decodesOnly(START + 2 * SECOND))
        assertFalse(probe.decodesOnly(START + 2 * SECOND + 1))
    }

    @Test
    fun `then one frame a second is only decoded`() {
        val probe = DecodeOnlyProbe(frames = 5)
        probe.decodesOnly(START)
        probe.decodesOnly(START + 2 * SECOND)

        assertFalse(probe.decodesOnly(START + 3 * SECOND - 1))
        assertTrue(probe.decodesOnly(START + 3 * SECOND))
    }

    @Test
    fun `a frame long overdue is only decoded once and the next waits a second after it`() {
        val probe = DecodeOnlyProbe(frames = 5)
        probe.decodesOnly(START)

        assertTrue(probe.decodesOnly(START + 4 * SECOND))
        assertFalse(probe.decodesOnly(START + 4 * SECOND + 1))
        assertFalse(probe.decodesOnly(START + 5 * SECOND - 1))
        assertTrue(probe.decodesOnly(START + 5 * SECOND))
    }

    @Test
    fun `the probe stops after its frames`() {
        val probe = DecodeOnlyProbe(frames = 2)

        val decodedOnly = (0..20).count { probe.decodesOnly(START + it * HALF_SECOND) }

        assertEquals(2, decodedOnly)
    }

    @Test
    fun `a restarted probe starts over`() {
        val probe = DecodeOnlyProbe(frames = 1)
        probe.decodesOnly(START)
        probe.decodesOnly(START + 2 * SECOND)

        probe.restart()

        assertFalse(probe.decodesOnly(START + 10 * SECOND))
        assertFalse(probe.decodesOnly(START + 11 * SECOND))
        assertTrue(probe.decodesOnly(START + 12 * SECOND))
    }

    private companion object {
        const val SECOND = 1_000_000_000L
        const val HALF_SECOND = SECOND / 2
        const val START = 7 * SECOND
    }
}
