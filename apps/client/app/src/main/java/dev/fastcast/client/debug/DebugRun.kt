package dev.fastcast.client.debug

import dev.fastcast.client.decoder.AudioStart
import dev.fastcast.client.decoder.DecoderPace
import dev.fastcast.client.decoder.DecoderSetup
import dev.fastcast.client.decoder.DecoderTweak
import dev.fastcast.client.decoder.TunnelClock
import dev.fastcast.client.decoder.TunnelTiming
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
        hideThenWarmUp {
            sampleVideo(STREAM_SAMPLE_MS) { measurement ->
                queue(STREAM, describe(measurement))
                runTrial(0)
            }
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
            host.isPlaying() -> warmUp(TRIALS[index]) { sampleTrial(index, asked) }
            clock.nowNanos() - askedNanos >= TRIAL_START_TIMEOUT_MS * NANOS_PER_MILLISECOND -> {
                val failure = listOf(asked, NO_TRIAL_VIDEO) + listOfNotNull(host.lastError())
                queue(TRIALS[index].step, failure + host.trialFacts())
                runTrial(index + 1)
            }
            else -> later(POLL_MS) { awaitTrialVideo(index, asked, askedNanos) }
        }
    }

    private fun warmUp(trial: StreamTrial, then: () -> Unit) {
        if (trial.showsText) later(WARMUP_MS, then) else hideThenWarmUp(then)
    }

    private fun hideThenWarmUp(then: () -> Unit) {
        host.hideProgress()
        later(WARMUP_MS, then)
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
        private const val WARMUP_MS = 3_000L
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
        private const val SHORT_VIDEO_DELAY_US = 25_000L
        private const val SHORTEST_VIDEO_DELAY_US = 12_000L
        private val ASKED_RATES = floatArrayOf(120f, 144f)
        private val TRIALS = listOf(
            tunnelAt1440("tunnel1440", DecoderSetup.USUAL, showsText = true),
            tunnelAt1440("tunnel1440hidden", DecoderSetup.USUAL, showsText = false),
            tunnelAt1440(
                "tunnel1440videofirst", timed(TunnelClock.VIDEO_DELAY_US, AudioStart.WITH_FIRST_FRAME), showsText = true,
            ),
            tunnelAt1440("tunnel1440delay25", timed(SHORT_VIDEO_DELAY_US, AudioStart.WITH_DECODER), showsText = true),
            tunnelAt1440("tunnel1440delay12", timed(SHORTEST_VIDEO_DELAY_US, AudioStart.WITH_DECODER), showsText = true),
            StreamTrial(
                "tunnel1080hidden", 1920, 1080, 120, tunneled = true, DecoderSetup.USUAL, showsText = false,
                DebugStep.TUNNEL,
            ),
            unpacedTunnel("tunnel1800", 3200, 1800, DecoderTweak.NONE),
            unpacedTunnel("tunnel2160", 3840, 2160, DecoderTweak.NONE),
            unpacedTunnel("tunnel2160game", 3840, 2160, DecoderTweak.MEDIATEK_GAME),
            unpacedTunnel("tunnel2160gamelive", 3840, 2160, DecoderTweak.MEDIATEK_GAME_LIVE),
            unpacedTunnel("tunnel2160realtime", 3840, 2160, DecoderTweak.MEDIATEK_REAL_TIME),
            StreamTrial(
                "fast2160gamelive", 3840, 2160, 120, tunneled = false, unpaced(DecoderTweak.MEDIATEK_GAME_LIVE),
                showsText = false, DebugStep.FAST,
            ),
        )

        private fun tunnelAt1440(step: String, setup: DecoderSetup, showsText: Boolean): StreamTrial =
            StreamTrial(step, 2560, 1440, 120, tunneled = true, setup, showsText, DebugStep.TUNNEL)

        private fun unpacedTunnel(step: String, width: Int, height: Int, tweak: DecoderTweak): StreamTrial =
            StreamTrial(step, width, height, 120, tunneled = true, unpaced(tweak), showsText = true, DebugStep.TUNNEL)

        private fun timed(videoDelayUs: Long, audioStart: AudioStart): DecoderSetup = DecoderSetup(
            DecoderPace.REALTIME, DecoderTweak.NONE, inspectsVendorParameters = false,
            TunnelTiming(videoDelayUs, audioStart),
        )

        private fun unpaced(tweak: DecoderTweak): DecoderSetup =
            DecoderSetup(DecoderPace.UNDECLARED, tweak, inspectsVendorParameters = true)
    }
}
