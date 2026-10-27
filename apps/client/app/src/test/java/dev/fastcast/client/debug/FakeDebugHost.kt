package dev.fastcast.client.debug

class FakeDebugHost(private val clock: FakeDebugClock) : DebugHost {

    val events = mutableListOf<String>()
    val sent = mutableListOf<String>()
    var playing = true
    var trialStartsAfterMs: Map<String, Long?> = emptyMap()
    var presentIntervalNanos = 8_333_333L
    var lastError: String? = null
    var trialFacts = listOf("audio played 6.0 s")
    var shownResults: List<TrialResult>? = null
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
        return DebugCounters(clock.nowNanos(), clock.nowMs * BYTES_PER_MILLISECOND, frames, frames, frames, 0L)
    }

    override fun presentedSince(startNanos: Long): LongArray =
        generateSequence(startNanos) { it + presentIntervalNanos }
            .takeWhile { it <= clock.nowNanos() }
            .toList()
            .toLongArray()

    override fun streamTrial(trial: StreamTrial): String {
        events += "trial ${trial.step}"
        playing = false
        val startsAfterMs = if (trial.step in trialStartsAfterMs) trialStartsAfterMs[trial.step] else TRIAL_START_MS
        startsAfterMs?.let { clock.after(it) { playing = true } }
        val path = if (trial.tunneled) " tunneled" else ""
        return "asked ${trial.width}x${trial.height}@${trial.fps}$path"
    }

    override fun trialFacts(): List<String> = trialFacts

    override fun lastError(): String? = lastError

    override fun streamUsual() {
        events += "usual"
        playing = false
        clock.after(USUAL_START_MS) { playing = true }
    }

    override fun showProgress(step: DebugStep) {
        events += "step ${step.name}"
    }

    override fun showDone(results: List<TrialResult>) {
        events += "done"
        shownResults = results
    }

    companion object {
        private const val TRIAL_START_MS = 2_000L
        private const val USUAL_START_MS = 1_000L
        private const val VSYNC_TICKS = 121
        private const val VSYNC_NANOS = 16_666_667L
        private const val DECODED_PER_SECOND = 120L
        private const val MILLISECONDS_PER_SECOND = 1_000L
        private const val BYTES_PER_MILLISECOND = 5_000L
    }
}
