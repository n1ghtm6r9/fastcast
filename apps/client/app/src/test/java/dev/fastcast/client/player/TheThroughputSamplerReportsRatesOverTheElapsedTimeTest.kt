package dev.fastcast.client.player

import org.junit.Assert.assertEquals
import org.junit.Test

class TheThroughputSamplerReportsRatesOverTheElapsedTimeTest {

    private val sampler = ThroughputSampler(startMs = 10_000L)

    @Test
    fun `sixty frames in one second are sixty frames per second`() {
        val rates = sampler.sample(nowMs = 11_000L, totalBytes = 0L, totalFrames = 60L)

        assertEquals(60.0, rates.framesPerSecond, 1e-9)
    }

    @Test
    fun `one and a quarter megabytes in one second are ten megabits per second`() {
        val rates = sampler.sample(nowMs = 11_000L, totalBytes = 1_250_000L, totalFrames = 0L)

        assertEquals(10.0, rates.megabitsPerSecond, 1e-9)
    }

    @Test
    fun `only the change since the previous sample counts`() {
        sampler.sample(nowMs = 11_000L, totalBytes = 1_250_000L, totalFrames = 60L)

        val rates = sampler.sample(nowMs = 12_000L, totalBytes = 1_875_000L, totalFrames = 90L)

        assertEquals(30.0, rates.framesPerSecond, 1e-9)
        assertEquals(5.0, rates.megabitsPerSecond, 1e-9)
    }

    @Test
    fun `a shorter interval scales the rates up`() {
        val rates = sampler.sample(nowMs = 10_500L, totalBytes = 625_000L, totalFrames = 30L)

        assertEquals(60.0, rates.framesPerSecond, 1e-9)
        assertEquals(10.0, rates.megabitsPerSecond, 1e-9)
    }
}
