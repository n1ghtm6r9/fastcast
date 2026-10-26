package dev.fastcast.client.player

import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.Surface
import dev.fastcast.client.decoder.DecoderEvents
import dev.fastcast.client.decoder.VideoDecoder
import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.stream.NativeStreamClient
import dev.fastcast.client.stream.StreamListener

class PlayerSession(
    private val offer: StreamOffer,
    surface: Surface,
    private val listener: PlayerSessionListener,
    displayLabel: () -> String,
) : StreamListener, DecoderEvents {

    private val mainHandler = Handler(Looper.getMainLooper())
    private val stream = NativeStreamClient(offer::target, this)
    private val decoder = VideoDecoder(surface, this)
    private val throughput = ThroughputSampler(SystemClock.elapsedRealtime())
    private val reportedThroughput = ThroughputSampler(SystemClock.elapsedRealtime())
    private val reporter = StatsReporter(
        snapshot = { snapshotOf(reportedThroughput, SystemClock.elapsedRealtime()) },
        display = displayLabel,
        send = stream::send,
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

    override fun onConnecting() = postStatus(PlayerStatus.Connecting)

    override fun onConnected() = postStatus(PlayerStatus.WaitingForVideo)

    override fun onConfig(config: ConfigMessage) {
        offer.codecConfigured(config.codec)
        decoder.submit(config)
    }

    override fun onFrame(frame: FrameMessage) = decoder.submit(frame)

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

    override fun onFailure(message: String) {
        offer.decoderFailed()
        postStatus(PlayerStatus.DecoderError(message))
        reporter.reportFailure(message, stream::reconnect)
    }

    private fun snapshotOf(sampler: ThroughputSampler, nowMs: Long): StatsSnapshot {
        val rates = sampler.sample(nowMs, stream.bytesReceived.get(), decoder.stats.framesRendered.get())
        return StatsSnapshot(
            width = videoWidth,
            height = videoHeight,
            framesPerSecond = rates.framesPerSecond,
            megabitsPerSecond = rates.megabitsPerSecond,
            decoderName = decoder.stats.decoderName,
            droppedFrames = decoder.stats.framesDropped.get(),
            mime = decoder.stats.mime,
            color = decoder.stats.outputColor,
        )
    }

    private fun postStatus(status: PlayerStatus) {
        mainHandler.post { if (active) listener.onStatus(status) }
    }
}
