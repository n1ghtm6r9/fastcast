package dev.fastcast.client.stream

import java.util.concurrent.Executors
import java.util.concurrent.RejectedExecutionException
import java.util.concurrent.ScheduledFuture
import java.util.concurrent.ThreadFactory
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicLong
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response
import okhttp3.WebSocket
import okhttp3.WebSocketListener
import okio.ByteString

class StreamClient(
    private val url: String,
    private val listener: StreamListener,
    private val backoff: ReconnectBackoff = ReconnectBackoff(),
    private val silenceLimitMs: Long = SILENCE_LIMIT_MS,
) {

    val bytesReceived = AtomicLong()

    private val lock = Any()
    private val silence = SilenceTimer(silenceLimitMs)
    private val httpClient = OkHttpClient.Builder()
        .connectTimeout(CONNECT_TIMEOUT_SECONDS, TimeUnit.SECONDS)
        .build()
    private val scheduler = Executors.newSingleThreadScheduledExecutor(
        ThreadFactory { task -> Thread(task, THREAD_NAME).apply { isDaemon = true } },
    )
    private var socket: WebSocket? = null
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
        openSocketIfRunning()
    }

    fun send(text: String): Boolean = synchronized(lock) { socket?.send(text) == true }

    fun reconnect() {
        synchronized(lock) {
            if (!running) return
            retireSocket()
            scheduleReopen(backoff.next())
        }
    }

    fun stop() {
        synchronized(lock) {
            running = false
            retireSocket()
        }
        scheduler.shutdownNow()
        httpClient.dispatcher.executorService.shutdown()
        httpClient.connectionPool.evictAll()
    }

    private fun openSocketIfRunning() {
        synchronized(lock) {
            if (running) openSocket()
        }
    }

    private fun openSocket() {
        socket?.cancel()
        generation++
        silence.touch()
        val request = Request.Builder().url(url).build()
        socket = httpClient.newWebSocket(request, SocketEvents(generation))
    }

    private fun watchForSilence() {
        val periodMs = silenceLimitMs / SILENCE_CHECKS_PER_LIMIT
        scheduler.scheduleWithFixedDelay({ dropIfSilent() }, periodMs, periodMs, TimeUnit.MILLISECONDS)
    }

    private fun dropIfSilent() {
        val delayMs = synchronized(lock) {
            val current = socket ?: return
            if (!silence.hasExpired()) return
            current.cancel()
            abandonSocket()
        }
        listener.onDisconnected(delayMs, NO_DATA)
    }

    private fun retireSocket() {
        generation++
        pendingReopen?.cancel(false)
        pendingReopen = null
        socket?.cancel()
        socket = null
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
        openSocketIfRunning()
    }

    private fun isCurrent(id: Int): Boolean = running && id == generation

    private fun lost(id: Int, reason: String) {
        val delayMs = synchronized(lock) {
            if (!running || id != generation) return
            abandonSocket()
        }
        listener.onDisconnected(delayMs, reason)
    }

    private fun abandonSocket(): Long {
        generation++
        socket = null
        return backoff.next().also(::scheduleReopen)
    }

    private inner class SocketEvents(private val id: Int) : WebSocketListener() {

        override fun onOpen(webSocket: WebSocket, response: Response) {
            if (!isCurrent(id)) return
            silence.touch()
            listener.onConnected()
        }

        override fun onMessage(webSocket: WebSocket, text: String) {
            if (!isCurrent(id)) return
            silence.touch()
            ConfigMessage.parse(text)?.let(listener::onConfig)
        }

        override fun onMessage(webSocket: WebSocket, bytes: ByteString) {
            if (!isCurrent(id)) return
            silence.touch()
            bytesReceived.addAndGet(bytes.size.toLong())
            val frame = FrameMessage.parse(bytes.toByteArray()) ?: return
            backoff.reset()
            listener.onFrame(frame)
        }

        override fun onClosing(webSocket: WebSocket, code: Int, reason: String) {
            webSocket.close(NORMAL_CLOSURE, null)
            lost(id, reason.ifEmpty { CLOSED_BY_SERVER })
        }

        override fun onClosed(webSocket: WebSocket, code: Int, reason: String) {
            lost(id, reason.ifEmpty { CLOSED_BY_SERVER })
        }

        override fun onFailure(webSocket: WebSocket, t: Throwable, response: Response?) {
            lost(id, t.message ?: t.javaClass.simpleName)
        }
    }

    private companion object {
        const val CONNECT_TIMEOUT_SECONDS = 4L
        const val NORMAL_CLOSURE = 1000
        const val THREAD_NAME = "fastcast-stream"
        const val CLOSED_BY_SERVER = "Closed by server"
        const val NO_DATA = "No data from server"
        const val SILENCE_LIMIT_MS = 5_000L
        const val SILENCE_CHECKS_PER_LIMIT = 5
    }
}
