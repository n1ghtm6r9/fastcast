package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheTunnelVideoStartsOnTheRunningAudioClockTest {

    private val start = TunnelStart(VideoStart.ON_RUNNING_CLOCK.clockWaitNanos)

    @Test
    fun `a frame that comes before the audio clock runs is skipped`() {
        assertEquals(TunnelStep.SKIP, start.stepFor(keyframe = false, clockRuns = false, nowNanos = 0L))
    }

    @Test
    fun `the first keyframe before the audio clock runs is queued to wake the tunnel`() {
        assertEquals(TunnelStep.QUEUE, start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L))
    }

    @Test
    fun `frames after the waking keyframe are skipped while the clock stands`() {
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L)

        assertEquals(
            listOf(TunnelStep.SKIP, TunnelStep.SKIP),
            listOf(
                start.stepFor(keyframe = false, clockRuns = false, nowNanos = MS),
                start.stepFor(keyframe = true, clockRuns = false, nowNanos = 2 * MS),
            ),
        )
    }

    @Test
    fun `a keyframe on the running clock starts the video`() {
        assertEquals(TunnelStep.QUEUE, start.stepFor(keyframe = true, clockRuns = true, nowNanos = 0L))
    }

    @Test
    fun `a started video queues every frame even when the clock cannot be read`() {
        start.stepFor(keyframe = true, clockRuns = true, nowNanos = 0L)

        assertEquals(TunnelStep.QUEUE, start.stepFor(keyframe = false, clockRuns = false, nowNanos = MS))
    }

    @Test
    fun `the first frame on the running clock after skipped ones asks for a keyframe`() {
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L)
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = MS)

        assertEquals(TunnelStep.ASK_FOR_KEYFRAME, start.stepFor(keyframe = false, clockRuns = true, nowNanos = 2 * MS))
    }

    @Test
    fun `frames after the ask are skipped until the keyframe comes`() {
        start.stepFor(keyframe = false, clockRuns = true, nowNanos = 0L)

        assertEquals(
            listOf(TunnelStep.SKIP, TunnelStep.QUEUE, TunnelStep.QUEUE),
            listOf(
                start.stepFor(keyframe = false, clockRuns = true, nowNanos = MS),
                start.stepFor(keyframe = true, clockRuns = true, nowNanos = 2 * MS),
                start.stepFor(keyframe = false, clockRuns = true, nowNanos = 3 * MS),
            ),
        )
    }

    @Test
    fun `a clock that does not run within a second of the first frame is no longer waited for`() {
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = 0L)

        assertEquals(
            listOf(TunnelStep.SKIP, TunnelStep.ASK_FOR_KEYFRAME, TunnelStep.QUEUE),
            listOf(
                start.stepFor(keyframe = false, clockRuns = false, nowNanos = 999 * MS),
                start.stepFor(keyframe = false, clockRuns = false, nowNanos = 1_000 * MS),
                start.stepFor(keyframe = true, clockRuns = false, nowNanos = 1_001 * MS),
            ),
        )
    }

    @Test
    fun `a restart waits for the running clock again`() {
        start.stepFor(keyframe = true, clockRuns = true, nowNanos = 0L)

        start.restart()

        assertEquals(TunnelStep.SKIP, start.stepFor(keyframe = false, clockRuns = false, nowNanos = 5_000 * MS))
    }

    @Test
    fun `a restart lets the next wait wake the tunnel and ask for a keyframe again`() {
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L)
        start.stepFor(keyframe = false, clockRuns = true, nowNanos = MS)

        start.restart()

        assertEquals(
            listOf(TunnelStep.QUEUE, TunnelStep.ASK_FOR_KEYFRAME),
            listOf(
                start.stepFor(keyframe = true, clockRuns = false, nowNanos = 2 * MS),
                start.stepFor(keyframe = false, clockRuns = true, nowNanos = 3 * MS),
            ),
        )
    }

    @Test
    fun `a waiting video tells how many frames it skipped`() {
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L)
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = MS)
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = 2 * MS)

        assertEquals("video waits for the audio clock, 2 frames skipped", start.describe())
    }

    @Test
    fun `a video started on the running clock tells when it started and how many frames it skipped`() {
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L)
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = 8 * MS)
        start.stepFor(keyframe = false, clockRuns = true, nowNanos = 300 * MS)
        start.stepFor(keyframe = true, clockRuns = true, nowNanos = 320 * MS)

        assertEquals(
            "video started on the running audio clock 320 ms after its first frame, 2 frames skipped",
            start.describe(),
        )
    }

    @Test
    fun `a video started without the clock says so`() {
        start.stepFor(keyframe = false, clockRuns = false, nowNanos = 0L)
        start.stepFor(keyframe = true, clockRuns = false, nowNanos = 1_200 * MS)

        assertEquals(
            "video started without the audio clock 1200 ms after its first frame, 1 frames skipped",
            start.describe(),
        )
    }

    @Test
    fun `a video that starts at once queues its first keyframe without waiting for the clock`() {
        val atOnce = TunnelStart(VideoStart.AT_ONCE.clockWaitNanos)

        assertEquals(TunnelStep.QUEUE, atOnce.stepFor(keyframe = true, clockRuns = false, nowNanos = 0L))
        assertEquals("video started without the audio clock 0 ms after its first frame, 0 frames skipped", atOnce.describe())
    }

    private companion object {
        const val MS = 1_000_000L
    }
}
