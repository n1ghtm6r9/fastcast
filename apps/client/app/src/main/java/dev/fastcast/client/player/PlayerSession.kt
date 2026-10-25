package dev.fastcast.client.player

import android.os.Handler
import android.os.Looper
import android.os.SystemClock
import android.view.Surface
import dev.fastcast.client.decoder.DecoderEvents
import dev.fastcast.client.decoder.VideoDecoder
import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.stream.FrameMessage
import dev.fastcast.client.stream.StreamClient
import dev.fastcast.client.stream.StreamListener

class PlayerSession(
    address: String,
    surface: Surface,
    private val listener: PlayerSessionListener,
) : StreamListener, DecoderEvents {

    private val mainHandler = Handler(Looper.getMainLooper())
    private val stream = StreamClient("ws://$address/ws", this)
    private val decoder = VideoDecoder(surface, this)
    private val throughput = ThroughputSampler(SystemClock.elapsedRealtime())

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
    }

    fun stop() {
        active = false
        stream.stop()
        decoder.stop()
    }

    fun snapshot(nowMs: Long): StatsSnapshot {
        val rates = throughput.sample(nowMs, stream.bytesReceived.get(), decoder.stats.framesRendered.get())
        return StatsSnapshot(
            width = videoWidth,
            height = videoHeight,
            framesPerSecond = rates.framesPerSecond,
            megabitsPerSecond = rates.megabitsPerSecond,
            decoderName = decoder.stats.decoderName,
            droppedFrames = decoder.stats.framesDropped.get(),
        )
    }

    override fun onConnecting() = postStatus(PlayerStatus.Connecting)

    override fun onConnected() = postStatus(PlayerStatus.WaitingForVideo)

    override fun onConfig(config: ConfigMessage) = decoder.submit(config)

    override fun onFrame(frame: FrameMessage) = decoder.submit(frame)

    override fun onDisconnected(retryInMs: Long, reason: String) =
        postStatus(PlayerStatus.Reconnecting(retryInMs, reason))

    override fun onVideoSize(width: Int, height: Int) {
        videoWidth = width
        videoHeight = height
        mainHandler.post { if (active) listener.onVideoSize(width, height) }
    }

    override fun onFirstFrame() = postStatus(PlayerStatus.Playing)

    override fun onFailure(message: String) {
        postStatus(PlayerStatus.DecoderError(message))
        stream.reconnect()
    }

    private fun postStatus(status: PlayerStatus) {
        mainHandler.post { if (active) listener.onStatus(status) }
    }
}
