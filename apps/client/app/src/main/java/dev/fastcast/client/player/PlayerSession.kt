package dev.fastcast.client.player

import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.Surface
import dev.fastcast.client.debug.CodecParameterFacts
import dev.fastcast.client.debug.DebugCounters
import dev.fastcast.client.decoder.DecoderEvents
import dev.fastcast.client.decoder.DecoderSetup
import dev.fastcast.client.decoder.TunnelAudio
import dev.fastcast.client.decoder.VideoDecoder
import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.stream.DiagnoseMessage
import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.stream.NativeStreamClient
import dev.fastcast.client.stream.ScaleMessage
import dev.fastcast.client.stream.StreamListener

class PlayerSession(
    private val offer: StreamOffer,
    surface: Surface,
    private val listener: PlayerSessionListener,
    displayLabel: () -> String,
    networkLabel: () -> String?,
    tunnel: TunnelAudio? = null,
    setup: DecoderSetup = DecoderSetup.USUAL,
) : StreamListener, DecoderEvents {

    private val mainHandler = Handler(Looper.getMainLooper())
    private val stream = NativeStreamClient(offer::target, this)
    private val decoder = VideoDecoder(surface, this, tunnel = tunnel, setup = setup)
    private val tunneled = tunnel != null
    private val throughput = ThroughputSampler(SystemClock.elapsedRealtime())
    private val reportedThroughput = ThroughputSampler(SystemClock.elapsedRealtime())
    private val reporter = StatsReporter(
        snapshot = { snapshotOf(reportedThroughput, SystemClock.elapsedRealtime()) },
        display = displayLabel,
        send = stream::send,
        network = networkLabel,
    )

    @Volatile
    private var active = false

    @Volatile
    private var videoWidth = 0

    @Volatile
    private var videoHeight = 0

    fun start() {
        active = true
        decoder.start()
        stream.start()
        reporter.start()
    }

    fun stop() {
        active = false
        reporter.stop()
        stream.stop()
        decoder.stop()
    }

    fun snapshot(nowMs: Long): StatsSnapshot = snapshotOf(throughput, nowMs)

    fun askScale(step: Int): Boolean = stream.askScale(step)

    fun send(text: String): Boolean = stream.send(text)

    fun debugCounters(): DebugCounters = DebugCounters(
        nanos = System.nanoTime(),
        bytes = stream.bytesReceived.get(),
        queued = decoder.stats.framesQueued.get(),
        decoded = decoder.stats.framesDecoded.get(),
        rendered = decoder.stats.framesRendered.get(),
        dropped = decoder.stats.framesDropped.get(),
    )

    fun presentedSince(startNanos: Long): LongArray = decoder.stats.presented.since(startNanos)

    fun tunnelFacts(): List<String> = decoder.tunnelFacts()

    fun codecFacts(): List<String> =
        decoder.stats.codecParameters?.let { CodecParameterFacts.of(it.vendor, it.outputFormat) }.orEmpty()

    override fun onConnecting() = postStatus(PlayerStatus.Connecting)

    override fun onConnected() = postStatus(PlayerStatus.WaitingForVideo)

    override fun onConfig(config: ConfigMessage) {
        offer.codecConfigured(config.codec)
        decoder.submit(config)
    }

    override fun onFrame(frame: FrameMessage) = decoder.submit(frame)

    override fun onScale(scale: ScaleMessage) {
        mainHandler.post { if (active) listener.onScale(scale) }
    }

    override fun onDiagnose(diagnose: DiagnoseMessage) {
        mainHandler.post { if (active) listener.onDiagnose(diagnose.run) }
    }

    override fun onHealthy() = offer.streamHealthy()

    override fun onDisconnected(retryInMs: Long, reason: String) =
        postStatus(PlayerStatus.Reconnecting(retryInMs, reason))

    override fun onConfigured() = reporter.reportNow()

    override fun onVideoSize(width: Int, height: Int) {
        videoWidth = width
        videoHeight = height
        mainHandler.post { if (active) listener.onVideoSize(width, height) }
    }

    override fun onFirstFrame() = postStatus(PlayerStatus.Playing)

    override fun onKeyframeNeeded() = stream.requestKeyFrame()

    override fun onFailure(message: String) {
        offer.decoderFailed()
        postStatus(PlayerStatus.DecoderError(message))
        reporter.reportFailure(message, stream::reconnect)
    }

    private fun snapshotOf(sampler: ThroughputSampler, nowMs: Long): StatsSnapshot {
        val rates = sampler.sample(
            nowMs,
            stream.bytesReceived.get(),
            decoder.stats.framesPresented.get(),
            decoder.stats.framesRendered.get(),
        )
        return StatsSnapshot(
            width = videoWidth,
            height = videoHeight,
            framesPerSecond = rates.framesPerSecond,
            megabitsPerSecond = rates.megabitsPerSecond,
            decoderName = decoder.stats.decoderName,
            droppedFrames = decoder.stats.framesDropped.get(),
            mime = decoder.stats.mime,
            color = decoder.stats.outputColor,
            receiveBufferBytes = stream.receiveBufferBytes(),
            delayMs = decoder.stats.delays.medianMs(),
            tunneled = tunneled,
        )
    }

    private fun postStatus(status: PlayerStatus) {
        mainHandler.post { if (active) listener.onStatus(status) }
    }
}
