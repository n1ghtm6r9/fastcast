package dev.fastcast.client.debug

import android.view.Choreographer

object VsyncMeter {

    private const val NANOS_PER_MILLISECOND = 1_000_000L

    fun measure(durationMs: Long, done: (LongArray) -> Unit) {
        val choreographer = Choreographer.getInstance()
        val times = mutableListOf<Long>()
        val endNanos = System.nanoTime() + durationMs * NANOS_PER_MILLISECOND
        choreographer.postFrameCallback(object : Choreographer.FrameCallback {
            override fun doFrame(frameTimeNanos: Long) {
                times += frameTimeNanos
                if (frameTimeNanos < endNanos) choreographer.postFrameCallback(this) else done(times.toLongArray())
            }
        })
    }
}
