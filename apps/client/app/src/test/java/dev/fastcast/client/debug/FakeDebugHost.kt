package dev.fastcast.client.debug

class FakeDebugHost(private val clock: FakeDebugClock) : DebugHost {

    val events = mutableListOf<String>()
    val sent = mutableListOf<String>()
    var playing = true
    var fastStartsAfterMs: Long? = FAST_START_MS
    var presentIntervalNanos = 8_333_333L
    var shownResult: FastStreamResult? = null
        private set
    var sections = listOf(
        FactSection("device", listOf("model TV")),
        FactSection("display", listOf("mode 3840x2160@60")),
    )

    override fun factSections(): List<FactSection> = sections

    override fun measureVsync(durationMs: Long, done: (LongArray) -> Unit) {
        events += "vsync"
        val startNanos = clock.nowNanos()
        clock.after(durationMs) { done(LongArray(VSYNC_TICKS) { startNanos + it * VSYNC_NANOS }) }
    }

    override fun askRefreshRate(rate: Float) {
        events += "ask $rate"
    }

    override fun clearRefreshRate() {
        events += "clear"
    }

    override fun displayNow(): String = "now 60.0"

    override fun isPlaying(): Boolean = playing

    override fun send(message: String): Boolean {
        sent += message
        return true
    }

    override fun counters(): DebugCounters {
        val frames = clock.nowMs * DECODED_PER_SECOND / MILLISECONDS_PER_SECOND
        return DebugCounters(clock.nowNanos(), clock.nowMs * BYTES_PER_MILLISECOND, frames, frames, 0L)
    }

    override fun presentedSince(startNanos: Long): LongArray =
        generateSequence(startNanos) { it + presentIntervalNanos }
            .takeWhile { it <= clock.nowNanos() }
            .toList()
            .toLongArray()

    override fun streamFast(): String {
        events += "fast"
        playing = false
        fastStartsAfterMs?.let { clock.after(it) { playing = true } }
        return "asked 3840x2160@120"
    }

    override fun streamUsual() {
        events += "usual"
        playing = false
        clock.after(USUAL_START_MS) { playing = true }
    }

    override fun showProgress(step: DebugStep) {
        events += "step ${step.name}"
    }

    override fun showDone(result: FastStreamResult?) {
        events += "done"
        shownResult = result
    }

    companion object {
        private const val FAST_START_MS = 2_000L
        private const val USUAL_START_MS = 1_000L
        private const val VSYNC_TICKS = 121
        private const val VSYNC_NANOS = 16_666_667L
        private const val DECODED_PER_SECOND = 120L
        private const val MILLISECONDS_PER_SECOND = 1_000L
        private const val BYTES_PER_MILLISECOND = 5_000L
    }
}
