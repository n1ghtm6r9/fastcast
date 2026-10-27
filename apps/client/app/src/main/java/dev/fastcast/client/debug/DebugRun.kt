package dev.fastcast.client.debug

import java.util.ArrayDeque
import kotlin.math.roundToInt

class DebugRun(
    private val host: DebugHost,
    private val clock: DebugClock,
) {

    private val outbox = ArrayDeque<String>()
    private var started = false
    private var finishing = false
    private val results = mutableListOf<TrialResult>()

    var isRunning = false
        private set

    fun start() {
        if (started) return
        started = true
        isRunning = true
        collectFacts()
        pump()
    }

    fun cancel() {
        isRunning = false
    }

    private fun collectFacts() {
        host.showProgress(DebugStep.FACTS)
        host.factSections().forEach { queue(it.step, it.facts) }
        measureVsync()
    }

    private fun measureVsync() {
        host.showProgress(DebugStep.VSYNC)
        sampleVsync(VSYNC) { askRates() }
    }

    private fun askRates() {
        host.showProgress(DebugStep.ASK)
        askRate(0)
    }

    private fun askRate(index: Int) {
        val rate = ASKED_RATES.getOrNull(index) ?: return measureStream()
        host.askRefreshRate(rate)
        later(SETTLE_MS) { sampleVsync(ASK + rate.roundToInt()) { askRate(index + 1) } }
    }

    private fun sampleVsync(step: String, then: () -> Unit) {
        host.measureVsync(VSYNC_SAMPLE_MS) { times ->
            if (isRunning) {
                queue(step, listOf(host.displayNow(), FrameRhythm.of(times).describe(VSYNC)))
                then()
            }
        }
    }

    private fun measureStream() {
        host.clearRefreshRate()
        host.showProgress(DebugStep.STREAM)
        sampleVideo(STREAM_SAMPLE_MS) { measurement ->
            queue(STREAM, describe(measurement))
            runTrial(0)
        }
    }

    private fun runTrial(index: Int) {
        val trial = TRIALS.getOrNull(index) ?: return restore()
        host.showProgress(trial.progress)
        val asked = host.streamTrial(trial)
        val askedNanos = clock.nowNanos()
        later(POLL_MS) { awaitTrialVideo(index, asked, askedNanos) }
    }

    private fun awaitTrialVideo(index: Int, asked: String, askedNanos: Long) {
        when {
            host.isPlaying() -> later(TRIAL_WARMUP_MS) { sampleTrial(index, asked) }
            clock.nowNanos() - askedNanos >= TRIAL_START_TIMEOUT_MS * NANOS_PER_MILLISECOND -> {
                val failure = listOf(asked, NO_TRIAL_VIDEO) + listOfNotNull(host.lastError())
                queue(TRIALS[index].step, failure + host.trialFacts())
                runTrial(index + 1)
            }
            else -> later(POLL_MS) { awaitTrialVideo(index, asked, askedNanos) }
        }
    }

    private fun sampleTrial(index: Int, asked: String) {
        val trial = TRIALS[index]
        sampleVideo(TRIAL_SAMPLE_MS) { measurement ->
            queue(trial.step, listOf(asked) + describe(measurement) + host.trialFacts())
            measurement?.let { results += TrialResult(trial, it.rates.queuedPerSecond, it.presented.perSecond) }
            runTrial(index + 1)
        }
    }

    private fun sampleVideo(durationMs: Long, then: (VideoMeasurement?) -> Unit) {
        val from = host.counters()
        later(durationMs) {
            val to = host.counters()
            then(if (from != null && to != null) measure(from, to) else null)
        }
    }

    private fun measure(from: DebugCounters, to: DebugCounters): VideoMeasurement =
        VideoMeasurement(CounterRates.between(from, to), FrameRhythm.of(host.presentedSince(from.nanos)))

    private fun describe(measurement: VideoMeasurement?): List<String> {
        if (measurement == null) return listOf(NO_VIDEO)
        return listOf(measurement.rates.describe(), measurement.presented.describe(PRESENTED), host.displayNow())
    }

    private fun restore() {
        host.showProgress(DebugStep.RESTORE)
        host.streamUsual()
        queue(DONE, listOf(FINISHED))
        finishing = true
    }

    private fun queue(step: String, facts: List<String>) {
        outbox.addAll(DebugMessages.pack(step, facts))
    }

    private fun pump() {
        if (!isRunning) return
        sendWhilePlaying()
        if (finishing && outbox.isEmpty()) return finish()
        clock.after(PUMP_MS) { pump() }
    }

    private fun sendWhilePlaying() {
        repeat(MESSAGES_PER_PUMP) {
            if (!host.isPlaying()) return
            val message = outbox.peekFirst() ?: return
            if (!host.send(message)) return
            outbox.pollFirst()
        }
    }

    private fun finish() {
        isRunning = false
        host.showDone(results)
    }

    private fun later(delayMs: Long, task: () -> Unit) {
        clock.after(delayMs) { if (isRunning) task() }
    }

    companion object {
        private const val VSYNC_SAMPLE_MS = 2_000L
        private const val SETTLE_MS = 1_500L
        private const val STREAM_SAMPLE_MS = 5_000L
        private const val TRIAL_START_TIMEOUT_MS = 20_000L
        private const val TRIAL_WARMUP_MS = 3_000L
        private const val TRIAL_SAMPLE_MS = 6_000L
        private const val POLL_MS = 250L
        private const val PUMP_MS = 50L
        private const val MESSAGES_PER_PUMP = 4
        private const val NANOS_PER_MILLISECOND = 1_000_000L
        private const val VSYNC = "vsync"
        private const val ASK = "ask"
        private const val STREAM = "stream"
        private const val DONE = "done"
        private const val PRESENTED = "presented"
        private const val FINISHED = "finished"
        private const val NO_VIDEO = "no video"
        private const val NO_TRIAL_VIDEO = "no video within 20 s"
        private val ASKED_RATES = floatArrayOf(120f, 144f)
        private val TRIALS = listOf(
            StreamTrial("fast1440", 2560, 1440, 120, tunneled = false, progress = DebugStep.FAST),
            StreamTrial("tunnel1440", 2560, 1440, 120, tunneled = true, progress = DebugStep.TUNNEL),
            StreamTrial("tunnel1080", 1920, 1080, 120, tunneled = true, progress = DebugStep.TUNNEL),
        )
    }
}
