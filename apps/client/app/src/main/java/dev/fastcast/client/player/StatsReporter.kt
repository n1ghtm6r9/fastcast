package dev.fastcast.client.player

import dev.fastcast.client.decoder.runQuietly
import dev.fastcast.client.stream.StatsMessage
import java.util.concurrent.Executors
import java.util.concurrent.RejectedExecutionException
import java.util.concurrent.ThreadFactory
import java.util.concurrent.TimeUnit

class StatsReporter(
    private val snapshot: () -> StatsSnapshot,
    private val display: () -> String,
    private val send: (String) -> Boolean,
    private val intervalMs: Long = INTERVAL_MS,
    private val flushDelayMs: Long = FLUSH_DELAY_MS,
    private val network: () -> String? = { null },
) {

    private val executor = Executors.newSingleThreadScheduledExecutor(
        ThreadFactory { task -> Thread(task, THREAD_NAME).apply { isDaemon = true } },
    )

    fun start() {
        executor.scheduleWithFixedDelay({ report(null) }, intervalMs, intervalMs, TimeUnit.MILLISECONDS)
    }

    fun reportNow() {
        submit(0L) { report(null) }
    }

    fun reportFailure(error: String, afterwards: () -> Unit) {
        submit(0L) { report(error) }
        submit(flushDelayMs, afterwards)
    }

    fun stop() {
        executor.shutdownNow()
    }

    private fun submit(delayMs: Long, task: () -> Unit) {
        try {
            executor.schedule(Runnable { task() }, delayMs, TimeUnit.MILLISECONDS)
        } catch (e: RejectedExecutionException) {
            return
        }
    }

    private fun report(error: String?) {
        runQuietly {
            val current = snapshot()
            val message = StatsMessage(
                decoder = current.decoderName,
                mime = current.mime,
                width = current.width,
                height = current.height,
                fps = current.framesPerSecond,
                mbps = current.megabitsPerSecond,
                dropped = current.droppedFrames,
                display = display(),
                receiveBuffer = current.receiveBufferBytes.takeIf { it > 0 },
                colorStandard = current.color.standard,
                colorRange = current.color.range,
                colorTransfer = current.color.transfer,
                network = network(),
                error = error,
                delayMs = current.delayMs,
                tunneled = current.tunneled,
                socketDrops = current.socketDrops,
                inboxDrops = current.inboxDrops,
                linkPauseMs = current.linkPauseMs,
            )
            send(message.toJson())
        }
    }

    private companion object {
        const val INTERVAL_MS = 5_000L
        const val FLUSH_DELAY_MS = 250L
        const val THREAD_NAME = "fastcast-stats"
    }
}
