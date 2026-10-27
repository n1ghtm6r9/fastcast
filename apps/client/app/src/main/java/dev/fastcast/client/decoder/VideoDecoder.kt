package dev.fastcast.client.decoder

import android.media.MediaCodec
import android.media.MediaFormat
import android.os.Build
import android.os.Process
import android.util.Log
import android.view.Surface
import dev.fastcast.client.display.FrameRateHint
import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.stream.FrameMessage
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class VideoDecoder(
    private val surface: Surface,
    private val events: DecoderEvents,
    val stats: DecoderStats = DecoderStats(),
    private val tunnel: TunnelAudio? = null,
) {

    private val mailbox = LinkedBlockingQueue<Packet>()
    private val backlog = PacketBacklog(MAX_QUEUED_PACKETS, stats.framesDropped, events::onKeyframeNeeded)
    private val bufferInfo = MediaCodec.BufferInfo()

    private var thread: Thread? = null
    private var codec: MediaCodec? = null
    private var nalLengthSize = 0
    private var inputCapacity = 0
    private var configured: ConfigMessage? = null
    private var pending: VideoSample? = null
    private var codecRendered = false
    private var firstFrameRendered = false

    @Volatile
    private var running = false

    fun start() {
        running = true
        thread = Thread({ decodeLoop() }, THREAD_NAME).apply { start() }
    }

    fun submit(config: ConfigMessage) {
        mailbox.offer(Packet.Config(config))
    }

    fun submit(frame: FrameMessage) {
        mailbox.offer(Packet.Frame(frame))
    }

    fun stop() {
        running = false
        thread?.join(JOIN_TIMEOUT_MS)
        thread = null
    }

    fun tunnelFacts(): List<String> {
        val audio = tunnel ?: return emptyList()
        val ready = if (stats.tunnelFrameReady) "ready" else "not reported"
        return listOf("tunnel decoder ${stats.decoderName.ifEmpty { "none" }}", "first tunnel frame $ready", audio.describe())
    }

    private fun decodeLoop() {
        Process.setThreadPriority(Process.THREAD_PRIORITY_URGENT_DISPLAY)
        try {
            while (running) step()
        } finally {
            releaseCodec()
        }
    }

    private fun step() {
        try {
            collectPackets()
            prepareNextSample()
            feedCodec()
            drainCodec()
        } catch (e: Exception) {
            fail(e)
        }
    }

    private fun collectPackets() {
        val idle = pending == null && backlog.isEmpty
        var packet = if (idle) mailbox.poll(idleWaitMillis(), TimeUnit.MILLISECONDS) else mailbox.poll()
        while (packet != null) {
            backlog.add(packet)
            packet = mailbox.poll()
        }
    }

    private fun idleWaitMillis(): Long = if (codec == null) NO_CODEC_WAIT_MS else ACTIVE_WAIT_MS

    private fun prepareNextSample() {
        while (pending == null) {
            when (val packet = backlog.next() ?: return) {
                is Packet.Config -> configure(packet.config)
                is Packet.Frame -> pending = prepare(packet.frame)
            }
        }
    }

    private fun prepare(frame: FrameMessage): VideoSample? {
        if (codec == null) return discard()
        val sample = AnnexBConverter.convert(frame, nalLengthSize)
        if (sample == null || sample.size > inputCapacity) {
            backlog.awaitKeyframe()
            return discard()
        }
        return sample
    }

    private fun discard(): VideoSample? {
        stats.framesDropped.incrementAndGet()
        return null
    }

    private fun configure(config: ConfigMessage) {
        firstFrameRendered = false
        if (tunnel != null || ConfigChange.needsNewDecoder(configured, codecRendered, config)) {
            startCodec(config)
        } else {
            codec?.flush()
        }
        events.onConfigured()
    }

    private fun startCodec(config: ConfigMessage) {
        releaseCodec()
        pending = null
        val mime = VideoMime.forCodec(config.codec)
            ?: throw UnsupportedStreamException("Unsupported codec ${config.codec}")
        val parameters = CodecConfigurationParser.parse(mime, config.description)
            ?: throw UnsupportedStreamException("Invalid codec description")
        val choice = chooseDecoder(mime, VideoFormatFactory.probe(mime, config))
        val format = VideoFormatFactory.create(mime, config, parameters, choice, tunnel?.sessionId)
        val started = createCodec(choice.name, format, config.fps)
        codec = started
        configured = config
        nalLengthSize = parameters.nalLengthSize
        inputCapacity = inputCapacityOf(started, format)
        stats.decoderName = choice.name
        stats.mime = mime
        stats.outputColor = OutputColor.NONE
        Log.i(TAG, "Decoder ${choice.name} for ${config.codec} ${config.width}x${config.height}@${config.fps}")
        events.onVideoSize(config.width, config.height)
    }

    private fun chooseDecoder(mime: String, probe: MediaFormat): DecoderChoice = if (tunnel == null) {
        DecoderSelector.select(mime, probe) ?: throw UnsupportedStreamException("No decoder for $mime")
    } else {
        DecoderSelector.selectTunneled(mime, probe) ?: throw UnsupportedStreamException("No tunneled decoder for $mime")
    }

    private fun createCodec(name: String, format: MediaFormat, fps: Int): MediaCodec {
        val created = MediaCodec.createByCodecName(name)
        try {
            created.configure(format, surface, null, 0)
            created.setOnFrameRenderedListener({ _, ptsUs, nanoTime -> stats.framePresented(ptsUs, nanoTime) }, null)
            if (tunnel != null) watchFirstTunnelFrame(created)
            FrameRateHint.apply(surface, fps.toFloat())
            created.start()
        } catch (e: Exception) {
            runQuietly(created::release)
            throw e
        }
        return created
    }

    private fun watchFirstTunnelFrame(created: MediaCodec) {
        stats.tunnelFrameReady = false
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.S) {
            created.setOnFirstTunnelFrameReadyListener(null) { stats.tunnelFrameReady = true }
        }
    }

    private fun inputCapacityOf(started: MediaCodec, requested: MediaFormat): Int {
        val actual = started.inputFormat
        val source = if (actual.containsKey(MediaFormat.KEY_MAX_INPUT_SIZE)) actual else requested
        return source.getInteger(MediaFormat.KEY_MAX_INPUT_SIZE)
    }

    private fun feedCodec() {
        val sample = pending ?: return
        val active = codec ?: return
        val index = active.dequeueInputBuffer(INPUT_WAIT_US)
        if (index < 0) return
        val buffer = checkNotNull(active.getInputBuffer(index)) { "Input buffer $index is unavailable" }
        buffer.clear()
        buffer.put(sample.data, sample.offset, sample.size)
        val presentationTimeUs = presentationTimeOf(sample)
        active.queueInputBuffer(index, 0, sample.size, presentationTimeUs, flagsOf(sample))
        pending = null
        stats.frameQueued(presentationTimeUs, System.nanoTime())
        tunnel?.let { onTunnelFrameQueued(it, sample) }
    }

    private fun presentationTimeOf(sample: VideoSample): Long =
        if (tunnel == null) sample.presentationTimeUs else TunnelClock.videoTimeUs(sample.presentationTimeUs)

    private fun onTunnelFrameQueued(audio: TunnelAudio, sample: VideoSample) {
        audio.start(sample.presentationTimeUs)
        codecRendered = true
        announceFirstFrame()
    }

    private fun flagsOf(sample: VideoSample): Int = if (sample.keyframe) MediaCodec.BUFFER_FLAG_KEY_FRAME else 0

    private fun drainCodec() {
        val active = codec ?: return
        var newest = NO_BUFFER
        var waitUs = if (hasMoreInput()) 0L else OUTPUT_WAIT_US
        while (true) {
            val index = active.dequeueOutputBuffer(bufferInfo, waitUs)
            waitUs = 0L
            if (index == MediaCodec.INFO_TRY_AGAIN_LATER) break
            if (index == MediaCodec.INFO_OUTPUT_FORMAT_CHANGED) onOutputFormatChanged(active.outputFormat)
            if (index >= 0) {
                stats.framesDecoded.incrementAndGet()
                newest = supersede(active, newest, index)
            }
        }
        if (newest != NO_BUFFER) render(active, newest)
    }

    private fun hasMoreInput(): Boolean = pending != null || !backlog.isEmpty

    private fun supersede(active: MediaCodec, previous: Int, index: Int): Int {
        if (previous != NO_BUFFER) {
            active.releaseOutputBuffer(previous, false)
            stats.framesDropped.incrementAndGet()
        }
        return index
    }

    private fun render(active: MediaCodec, index: Int) {
        active.releaseOutputBuffer(index, true)
        codecRendered = true
        stats.framesRendered.incrementAndGet()
        announceFirstFrame()
    }

    private fun announceFirstFrame() {
        if (firstFrameRendered) return
        firstFrameRendered = true
        events.onFirstFrame()
    }

    private fun onOutputFormatChanged(format: MediaFormat) {
        stats.outputColor = OutputColor.of(format)
        reportSize(format)
    }

    private fun reportSize(format: MediaFormat) {
        val width = VisibleSize.width(format)
        val height = VisibleSize.height(format)
        if (width > 0 && height > 0) events.onVideoSize(width, height)
    }

    private fun fail(error: Exception) {
        Log.e(TAG, "Decoder failure", error)
        releaseCodec()
        pending = null
        backlog.clear()
        events.onFailure(FailureText.of(error))
    }

    private fun releaseCodec() {
        val active = codec ?: return
        codec = null
        configured = null
        codecRendered = false
        runQuietly(active::stop)
        runQuietly(active::release)
        tunnel?.stop()
    }

    private companion object {
        const val TAG = "VideoDecoder"
        const val THREAD_NAME = "fastcast-decoder"
        const val MAX_QUEUED_PACKETS = 30
        const val NO_CODEC_WAIT_MS = 20L
        const val ACTIVE_WAIT_MS = 1L
        const val INPUT_WAIT_US = 2_000L
        const val OUTPUT_WAIT_US = 2_000L
        const val JOIN_TIMEOUT_MS = 3_000L
        const val NO_BUFFER = -1
    }
}
