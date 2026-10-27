package dev.fastcast.client.debug

import android.os.Handler

class HandlerClock(private val handler: Handler) : DebugClock {

    override fun nowNanos(): Long = System.nanoTime()

    override fun after(delayMs: Long, task: () -> Unit) {
        handler.postDelayed({ task() }, delayMs)
    }
}
