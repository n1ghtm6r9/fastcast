package dev.fastcast.client.stream

import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.CopyOnWriteArrayList
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit
import java.util.concurrent.atomic.AtomicInteger
import java.util.concurrent.atomic.AtomicLong

class FakeStreamLink : StreamLink {

    private val lastHandle = AtomicLong()
    private val refusals = AtomicInteger()
    private val inboxes = ConcurrentHashMap<Long, LinkedBlockingQueue<ByteArray>>()
    private val outboxes = ConcurrentHashMap<Long, LinkedBlockingQueue<ByteArray>>()
    private val opened = LinkedBlockingQueue<Long>()
    private val openedTargets = CopyOnWriteArrayList<StreamTarget>()
    private val closedLinks = CopyOnWriteArrayList<Long>()

    val targets: List<StreamTarget>
        get() = openedTargets.toList()

    val closedHandles: List<Long>
        get() = closedLinks.toList()

    fun refuseNextOpens(count: Int) = refusals.set(count)

    fun accept(): Long = acceptWithin(ACCEPT_TIMEOUT_MS) ?: error("No link was opened in time")

    fun acceptWithin(timeoutMs: Long): Long? = opened.poll(timeoutMs, TimeUnit.MILLISECONDS)

    fun acceptConnected(): Long = accept().also(::connect)

    fun connect(handle: Long) = push(handle, NativeLink.EVENT_CONNECTED, ByteArray(0))

    fun config(handle: Long, json: String) = push(handle, NativeLink.EVENT_CONFIG, json.toByteArray(Charsets.UTF_8))

    fun frame(handle: Long, bytes: ByteArray) = push(handle, NativeLink.EVENT_VIDEO, bytes)

    fun end(handle: Long, reason: String) = push(handle, NativeLink.EVENT_ENDED, reason.toByteArray(Charsets.UTF_8))

    fun raw(handle: Long, event: ByteArray) {
        inboxes[handle]?.add(event)
    }

    fun nextStats(handle: Long): String {
        val outbox = outboxes[handle] ?: error("No open link $handle")
        val text = outbox.poll(STATS_TIMEOUT_MS, TimeUnit.MILLISECONDS) ?: error("No stats were sent in time")
        return String(text, Charsets.UTF_8)
    }

    override fun open(target: StreamTarget): Long {
        openedTargets.add(target)
        if (refusals.getAndUpdate { maxOf(it - 1, 0) } > 0) return NO_HANDLE
        val handle = lastHandle.incrementAndGet()
        inboxes[handle] = LinkedBlockingQueue()
        outboxes[handle] = LinkedBlockingQueue()
        opened.add(handle)
        return handle
    }

    override fun receive(link: Long, timeoutMs: Int): ByteArray? =
        inboxes[link]?.poll(timeoutMs.toLong(), TimeUnit.MILLISECONDS)

    override fun sendStats(link: Long, text: ByteArray): Boolean {
        val outbox = outboxes[link] ?: return false
        outbox.add(text)
        return true
    }

    override fun close(link: Long) {
        closedLinks.add(link)
        inboxes.remove(link)
        outboxes.remove(link)
    }

    private fun push(handle: Long, kind: Int, payload: ByteArray) {
        inboxes[handle]?.add(byteArrayOf(kind.toByte()) + payload)
    }

    private companion object {
        const val NO_HANDLE = 0L
        const val ACCEPT_TIMEOUT_MS = 5_000L
        const val STATS_TIMEOUT_MS = 5_000L
    }
}
