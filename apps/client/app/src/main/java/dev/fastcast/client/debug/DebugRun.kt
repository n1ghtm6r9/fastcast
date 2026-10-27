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
    private var result: FastStreamResult? = null

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
        sampleVideo(STREAM_SAMPLE_MS, STREAM, emptyList()) { streamFast() }
    }

    private fun streamFast() {
        host.showProgress(DebugStep.FAST)
        val asked = host.streamFast()
        val askedNanos = clock.nowNanos()
        later(POLL_MS) { awaitFastVideo(asked, askedNanos) }
    }

    private fun awaitFastVideo(asked: String, askedNanos: Long) {
        when {
            host.isPlaying() -> later(FAST_WARMUP_MS) { sampleFastVideo(asked) }
            clock.nowNanos() - askedNanos >= FAST_START_TIMEOUT_MS * NANOS_PER_MILLISECOND -> {
                queue(FAST, listOf(asked, NO_FAST_VIDEO))
                restore()
            }
            else -> later(POLL_MS) { awaitFastVideo(asked, askedNanos) }
        }
    }

    private fun sampleFastVideo(asked: String) {
        sampleVideo(FAST_SAMPLE_MS, FAST, listOf(asked)) { sample ->
            result = sample
            restore()
        }
    }

    private fun sampleVideo(durationMs: Long, step: String, lead: List<String>, then: (FastStreamResult?) -> Unit) {
        val from = host.counters()
        later(durationMs) {
            val to = host.counters()
            if (from == null || to == null) {
                queue(step, lead + NO_VIDEO)
                then(null)
            } else {
                val rates = CounterRates.between(from, to)
                val presented = FrameRhythm.of(host.presentedSince(from.nanos))
                queue(step, lead + listOf(rates.describe(), presented.describe(PRESENTED), host.displayNow()))
                then(FastStreamResult(rates.decodedPerSecond, presented.perSecond))
            }
        }
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
        host.showDone(result)
    }

    private fun later(delayMs: Long, task: () -> Unit) {
        clock.after(delayMs) { if (isRunning) task() }
    }

    companion object {
        private const val VSYNC_SAMPLE_MS = 2_000L
        private const val SETTLE_MS = 1_500L
        private const val STREAM_SAMPLE_MS = 5_000L
        private const val FAST_START_TIMEOUT_MS = 20_000L
        private const val FAST_WARMUP_MS = 3_000L
        private const val FAST_SAMPLE_MS = 6_000L
        private const val POLL_MS = 250L
        private const val PUMP_MS = 50L
        private const val MESSAGES_PER_PUMP = 4
        private const val NANOS_PER_MILLISECOND = 1_000_000L
        private const val VSYNC = "vsync"
        private const val ASK = "ask"
        private const val STREAM = "stream"
        private const val FAST = "fast"
        private const val DONE = "done"
        private const val PRESENTED = "presented"
        private const val FINISHED = "finished"
        private const val NO_VIDEO = "no video"
        private const val NO_FAST_VIDEO = "no video within 20 s"
        private val ASKED_RATES = floatArrayOf(120f, 144f)
    }
}
