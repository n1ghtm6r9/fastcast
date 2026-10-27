package dev.fastcast.client.player

import org.junit.Assert.assertEquals
import org.junit.Test

class TheThroughputSamplerCountsWhatTheScreenShowedTest {

    private val sampler = ThroughputSampler(startMs = 10_000L)

    @Test
    fun `a window the screen reported counts the frames it showed`() {
        val rates = sampler.sample(nowMs = 11_000L, totalBytes = 0L, presentedFrames = 60L, renderedFrames = 120L)

        assertEquals(60.0, rates.framesPerSecond, 1e-9)
    }

    @Test
    fun `a window without screen reports counts the frames handed to the screen`() {
        val rates = sampler.sample(nowMs = 11_000L, totalBytes = 0L, presentedFrames = 0L, renderedFrames = 120L)

        assertEquals(120.0, rates.framesPerSecond, 1e-9)
    }

    @Test
    fun `a screen that stops reporting counts the frames handed to it from then on`() {
        sampler.sample(nowMs = 11_000L, totalBytes = 0L, presentedFrames = 60L, renderedFrames = 120L)

        val rates = sampler.sample(nowMs = 12_000L, totalBytes = 0L, presentedFrames = 60L, renderedFrames = 180L)

        assertEquals(60.0, rates.framesPerSecond, 1e-9)
    }
}
