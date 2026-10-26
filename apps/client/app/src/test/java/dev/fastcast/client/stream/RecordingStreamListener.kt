package dev.fastcast.client.stream

import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class RecordingStreamListener : StreamListener {

    private val events = LinkedBlockingQueue<String>()
    private val configs = LinkedBlockingQueue<ConfigMessage>()
    private val frames = LinkedBlockingQueue<FrameMessage>()
    private val retryDelays = LinkedBlockingQueue<Long>()
    private val reasons = LinkedBlockingQueue<String>()

    override fun onConnecting() {
        events.add(CONNECTING)
    }

    override fun onConnected() {
        events.add(CONNECTED)
    }

    override fun onConfig(config: ConfigMessage) {
        configs.add(config)
        events.add(CONFIG)
    }

    override fun onFrame(frame: FrameMessage) {
        frames.add(frame)
        events.add(FRAME)
    }

    override fun onHealthy() {
        events.add(HEALTHY)
    }

    override fun onDisconnected(retryInMs: Long, reason: String) {
        retryDelays.add(retryInMs)
        reasons.add(reason)
        events.add(DISCONNECTED)
    }

    fun nextEvent(): String = await(events)

    fun skipTo(event: String) {
        while (nextEvent() != event) continue
    }

    fun hasNoEventWithin(timeoutMs: Long): Boolean = events.poll(timeoutMs, TimeUnit.MILLISECONDS) == null

    fun nextConfig(): ConfigMessage = await(configs)

    fun nextFrame(): FrameMessage = await(frames)

    fun nextRetryDelay(): Long = await(retryDelays)

    fun nextReason(): String = await(reasons)

    private fun <T : Any> await(queue: LinkedBlockingQueue<T>): T =
        queue.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for a stream event")

    companion object {
        const val CONNECTING = "connecting"
        const val CONNECTED = "connected"
        const val CONFIG = "config"
        const val FRAME = "frame"
        const val HEALTHY = "healthy"
        const val DISCONNECTED = "disconnected"
        private const val TIMEOUT_SECONDS = 5L
    }
}
