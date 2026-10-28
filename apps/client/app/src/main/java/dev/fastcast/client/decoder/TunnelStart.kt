package dev.fastcast.client.decoder

import java.util.Locale

class TunnelStart(private val clockWaitNanos: Long) {

    private var firstFrameNanos: Long? = null
    private var woken = false
    private var asked = false

    @Volatile
    private var startedAfterNanos: Long? = null

    @Volatile
    private var startedOnClock = false

    @Volatile
    private var skipped = 0

    fun stepFor(keyframe: Boolean, clockRuns: Boolean, nowNanos: Long): TunnelStep {
        if (startedAfterNanos != null) return TunnelStep.QUEUE
        val waitedNanos = nowNanos - (firstFrameNanos ?: nowNanos.also { firstFrameNanos = it })
        val ready = clockRuns || waitedNanos >= clockWaitNanos
        val step = when {
            ready && keyframe -> start(waitedNanos, clockRuns)
            ready && !asked -> ask()
            keyframe && !woken -> wake()
            else -> TunnelStep.SKIP
        }
        if (step != TunnelStep.QUEUE) skipped++
        return step
    }

    fun restart() {
        firstFrameNanos = null
        woken = false
        asked = false
        startedAfterNanos = null
        startedOnClock = false
        skipped = 0
    }

    fun describe(): String {
        val afterNanos = startedAfterNanos ?: return WAITING.format(Locale.ROOT, skipped)
        val how = if (startedOnClock) ON_CLOCK else WITHOUT_CLOCK
        return STARTED.format(Locale.ROOT, how, afterNanos / NANOS_PER_MILLISECOND, skipped)
    }

    private fun start(waitedNanos: Long, onClock: Boolean): TunnelStep {
        startedOnClock = onClock
        startedAfterNanos = waitedNanos
        return TunnelStep.QUEUE
    }

    private fun ask(): TunnelStep {
        asked = true
        return TunnelStep.ASK_FOR_KEYFRAME
    }

    private fun wake(): TunnelStep {
        woken = true
        return TunnelStep.QUEUE
    }

    private companion object {
        const val NANOS_PER_MILLISECOND = 1_000_000L
        const val WAITING = "video waits for the audio clock, %d frames skipped"
        const val STARTED = "video started %s %d ms after its first frame, %d frames skipped"
        const val ON_CLOCK = "on the running audio clock"
        const val WITHOUT_CLOCK = "without the audio clock"
    }
}
