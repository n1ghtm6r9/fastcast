package dev.fastcast.client.player

import org.junit.Assert.assertEquals
import org.junit.Test

class TheThroughputSamplerReportsZeroWhenNoTimeHasPassedTest {

    private val sampler = ThroughputSampler(startMs = 5_000L)

    @Test
    fun `a sample taken at the start time reports zero rates`() {
        val rates = sampler.sample(nowMs = 5_000L, totalBytes = 1_000L, totalFrames = 10L)

        assertEquals(0.0, rates.framesPerSecond, 0.0)
        assertEquals(0.0, rates.megabitsPerSecond, 0.0)
    }

    @Test
    fun `a clock that moves backwards reports zero rates`() {
        val rates = sampler.sample(nowMs = 4_000L, totalBytes = 1_000L, totalFrames = 10L)

        assertEquals(0.0, rates.framesPerSecond, 0.0)
        assertEquals(0.0, rates.megabitsPerSecond, 0.0)
    }
}
