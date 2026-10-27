package dev.fastcast.client.stream

import dev.fastcast.client.stream.NativeLink.NO_HANDLE
import java.util.concurrent.Executors
import java.util.concurrent.RejectedExecutionException
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.ThreadFactory
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicLong

class NativeStreamClient(
    private val targetFor: () -> StreamTarget,
    private val listener: StreamListener,
    private val link: StreamLink = NativeStreamLink,
    private val backoff: ReconnectBackoff = ReconnectBackoff(),
    private val silence: SilenceTimer = SilenceTimer(),
    private val health: StreamHealth = StreamHealth(),
) {

    val bytesReceived = AtomicLong()

    private val lock = Any()
    private val scheduler = Executors.newSingleThreadScheduledExecutor(
        ThreadFactory { task -> Thread(task, THREAD_NAME).apply { isDaemon = true } },
    )
    private var handle = NO_HANDLE
    private var pendingReopen: ScheduledFuture<*>? = null

    @Volatile
    private var generation = 0

    @Volatile
    private var running = false

    fun start() {
        val started = synchronized(lock) {
            val wasRunning = running
            running = true
            !wasRunning
        }
        if (!started) return
        watchForSilence()
        listener.onConnecting()
        openLinkIfRunning()
    }

    fun send(text: String): Boolean = synchronized(lock) {
        handle != NO_HANDLE && link.sendStats(handle, text.toByteArray(Charsets.UTF_8))
    }

    fun askScale(step: Int): Boolean = synchronized(lock) {
        handle != NO_HANDLE && link.sendNotice(handle, ScaleAsk(step).toJson().toByteArray(Charsets.UTF_8))
    }

    fun receiveBufferBytes(): Int = synchronized(lock) {
        if (handle == NO_HANDLE) UNKNOWN_BUFFER else link.receiveBufferBytes(handle)
    }

    fun requestKeyFrame() {
        synchronized(lock) {
            if (handle != NO_HANDLE) link.requestKeyFrame(handle)
        }
    }

    fun reconnect() {
        synchronized(lock) {
            if (!running) return
            retireLink()
            scheduleReopen(backoff.next())
        }
    }

    fun stop() {
        synchronized(lock) {
            running = false
            retireLink()
        }
        scheduler.shutdownNow()
    }

    private fun openLinkIfRunning() {
        val refusedRetryMs = synchronized(lock) {
            if (!running) return
            openLink()
        }
        refusedRetryMs?.let { listener.onDisconnected(it, COULD_NOT_OPEN) }
    }

    private fun openLink(): Long? {
        closeCurrentLink()
        generation++
        silence.restart()
        health.restart()
        val opened = link.open(targetFor())
        if (opened == NO_HANDLE) return abandonLink()
        handle = opened
        startReader(generation, opened)
        return null
    }

    private fun startReader(id: Int, opened: Long) {
        Thread({ read(id, opened) }, READER_THREAD_NAME).apply { isDaemon = true }.start()
    }

    private fun read(id: Int, opened: Long) {
        while (isCurrent(id)) {
            val event = link.receive(opened, RECEIVE_POLL_MS) ?: continue
            dispatch(id, event)
        }
    }

    private fun dispatch(id: Int, event: ByteArray) {
        if (!isCurrent(id) || event.isEmpty()) return
        when (event[0].toInt()) {
            NativeLink.EVENT_CONNECTED -> connected()
            NativeLink.EVENT_CONFIG -> config(event)
            NativeLink.EVENT_VIDEO -> video(event)
            NativeLink.EVENT_ENDED -> lost(id, textAfterKind(event).ifEmpty { CLOSED_BY_SERVER })
            NativeLink.EVENT_NOTICE -> notice(event)
        }
    }

    private fun connected() {
        silence.restart()
        listener.onConnected()
    }

    private fun config(event: ByteArray) {
        silence.touch()
        ConfigMessage.parse(textAfterKind(event))?.let(listener::onConfig)
    }

    private fun video(event: ByteArray) {
        silence.touch()
        bytesReceived.addAndGet((event.size - KIND_SIZE).toLong())
        val frame = FrameMessage.parse(event, KIND_SIZE) ?: return
        if (health.frameDelivered()) {
            backoff.reset()
            listener.onHealthy()
        }
        listener.onFrame(frame)
    }

    private fun notice(event: ByteArray) {
        ScaleMessage.parse(textAfterKind(event))?.let(listener::onScale)
    }

    private fun textAfterKind(event: ByteArray): String =
        String(event, KIND_SIZE, event.size - KIND_SIZE, Charsets.UTF_8)

    private fun watchForSilence() {
        val periodMs = silence.checkPeriodMs
        scheduler.scheduleWithFixedDelay({ dropIfSilent() }, periodMs, periodMs, TimeUnit.MILLISECONDS)
    }

    private fun dropIfSilent() {
        val delayMs = synchronized(lock) {
            if (handle == NO_HANDLE) return
            if (!silence.hasExpired()) return
            abandonLink()
        }
        listener.onDisconnected(delayMs, NO_DATA)
    }

    private fun retireLink() {
        generation++
        pendingReopen?.cancel(false)
        pendingReopen = null
        closeCurrentLink()
    }

    private fun closeCurrentLink() {
        val closing = handle
        handle = NO_HANDLE
        if (closing != NO_HANDLE) link.close(closing)
    }

    private fun scheduleReopen(delayMs: Long) {
        pendingReopen = try {
            scheduler.schedule(Runnable { reopen() }, delayMs, TimeUnit.MILLISECONDS)
        } catch (e: RejectedExecutionException) {
            null
        }
    }

    private fun reopen() {
        if (!running) return
        listener.onConnecting()
        openLinkIfRunning()
    }

    private fun isCurrent(id: Int): Boolean = running && id == generation

    private fun lost(id: Int, reason: String) {
        val delayMs = synchronized(lock) {
            if (!running || id != generation) return
            abandonLink()
        }
        listener.onDisconnected(delayMs, reason)
    }

    private fun abandonLink(): Long {
        generation++
        closeCurrentLink()
        return backoff.next().also(::scheduleReopen)
    }

    private companion object {
        const val UNKNOWN_BUFFER = 0
        const val KIND_SIZE = 1
        const val RECEIVE_POLL_MS = 100
        const val THREAD_NAME = "fastcast-stream"
        const val READER_THREAD_NAME = "fastcast-stream-reader"
        const val CLOSED_BY_SERVER = "Closed by server"
        const val NO_DATA = "No data from server"
        const val COULD_NOT_OPEN = "Could not open the stream"
    }
}
