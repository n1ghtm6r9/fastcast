package dev.fastcast.client.debug

import org.junit.Assert.assertEquals
import org.junit.Test

class TheFrameRhythmTellsHowOftenFramesCameTest {

    private fun spaced(count: Int, stepNanos: Long): LongArray = LongArray(count) { START_NANOS + it * stepNanos }

    @Test
    fun `frames a hundred and twentieth of a second apart come 120 times a second`() {
        val rhythm = FrameRhythm.of(spaced(121, 8_333_333L))

        assertEquals(121, rhythm.frames)
        assertEquals(120.0, rhythm.perSecond, 0.01)
        assertEquals(8.333, rhythm.medianMs, 0.001)
    }

    @Test
    fun `the median of an even number of intervals lies between the middle two`() {
        val rhythm = FrameRhythm.of(longArrayOf(0L, 10_000_000L, 30_000_000L, 60_000_000L, 100_000_000L))

        assertEquals(25.0, rhythm.medianMs, 1e-9)
    }

    @Test
    fun `a rare long gap does not move the median`() {
        assertEquals(16.667, FrameRhythm.of(SIXTY_WITH_A_GAP).medianMs, 0.001)
    }

    @Test
    fun `intervals are counted by whole milliseconds with the most common first`() {
        assertEquals(listOf(17 to 4, 50 to 1), FrameRhythm.of(SIXTY_WITH_A_GAP).commonIntervals)
    }

    @Test
    fun `equally common intervals list the shorter one first`() {
        val rhythm = FrameRhythm.of(longArrayOf(0L, 20_000_000L, 30_000_000L))

        assertEquals(listOf(10 to 1, 20 to 1), rhythm.commonIntervals)
    }

    @Test
    fun `only the six most common intervals are listed`() {
        val times = LongArray(9) { index -> index * (index + 1) / 2 * 1_000_000L }

        assertEquals(listOf(1 to 1, 2 to 1, 3 to 1, 4 to 1, 5 to 1, 6 to 1), FrameRhythm.of(times).commonIntervals)
    }

    @Test
    fun `the description names the label, the rate, the median and the intervals`() {
        val description = FrameRhythm.of(spaced(121, 8_333_333L)).describe("presented")

        assertEquals("presented: 121 frames in 1.00 s = 120.0/s, median 8.33 ms, ms 8:120", description)
    }

    @Test
    fun `fewer than two frames have no rate`() {
        val rhythm = FrameRhythm.of(longArrayOf(START_NANOS))

        assertEquals(0.0, rhythm.perSecond, 0.0)
        assertEquals("vsync: 1 frames", rhythm.describe("vsync"))
    }

    companion object {
        private const val START_NANOS = 5_000_000_000L
        private val SIXTY_WITH_A_GAP =
            longArrayOf(0L, 16_666_667L, 33_333_333L, 50_000_000L, 66_666_667L, 116_666_667L)
    }
}
