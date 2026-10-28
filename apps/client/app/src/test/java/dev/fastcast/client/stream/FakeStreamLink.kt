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
    private val statsOutboxes = ConcurrentHashMap<Long, LinkedBlockingQueue<ByteArray>>()
    private val noticeOutboxes = ConcurrentHashMap<Long, LinkedBlockingQueue<ByteArray>>()
    private val opened = LinkedBlockingQueue<Long>()
    private val openedTargets = CopyOnWriteArrayList<StreamTarget>()
    private val closedLinks = CopyOnWriteArrayList<Long>()
    private val keyFrameRequests = CopyOnWriteArrayList<Long>()
    private val socketDropCounts = ConcurrentHashMap<Long, Int>()
    private val inboxDropCounts = ConcurrentHashMap<Long, Int>()
    private val longestPauses = ConcurrentHashMap<Long, Int>()

    val targets: List<StreamTarget>
        get() = openedTargets.toList()

    val closedHandles: List<Long>
        get() = closedLinks.toList()

    val keyFrameRequestedOn: List<Long>
        get() = keyFrameRequests.toList()

    fun refuseNextOpens(count: Int) = refusals.set(count)

    fun accept(): Long = acceptWithin(ACCEPT_TIMEOUT_MS) ?: error("No link was opened in time")

    fun acceptWithin(timeoutMs: Long): Long? = opened.poll(timeoutMs, TimeUnit.MILLISECONDS)

    fun acceptConnected(): Long = accept().also(::connect)

    fun connect(handle: Long) = push(handle, NativeLink.EVENT_CONNECTED, ByteArray(0))

    fun config(handle: Long, json: String) = push(handle, NativeLink.EVENT_CONFIG, json.toByteArray(Charsets.UTF_8))

    fun frame(handle: Long, bytes: ByteArray) = push(handle, NativeLink.EVENT_VIDEO, bytes)

    fun end(handle: Long, reason: String) = push(handle, NativeLink.EVENT_ENDED, reason.toByteArray(Charsets.UTF_8))

    fun notice(handle: Long, json: String) = push(handle, NativeLink.EVENT_NOTICE, json.toByteArray(Charsets.UTF_8))

    fun raw(handle: Long, event: ByteArray) {
        inboxes[handle]?.add(event)
    }

    fun nextStats(handle: Long): String = nextSent(statsOutboxes, handle)

    fun nextNotice(handle: Long): String = nextSent(noticeOutboxes, handle)

    override fun open(target: StreamTarget): Long {
        openedTargets.add(target)
        if (refusals.getAndUpdate { maxOf(it - 1, 0) } > 0) return NO_HANDLE
        val handle = lastHandle.incrementAndGet()
        inboxes[handle] = LinkedBlockingQueue()
        statsOutboxes[handle] = LinkedBlockingQueue()
        noticeOutboxes[handle] = LinkedBlockingQueue()
        opened.add(handle)
        return handle
    }

    override fun receive(link: Long, timeoutMs: Int): ByteArray? =
        inboxes[link]?.poll(timeoutMs.toLong(), TimeUnit.MILLISECONDS)

    override fun sendStats(link: Long, text: ByteArray): Boolean = statsOutboxes[link]?.add(text) ?: false

    override fun sendNotice(link: Long, text: ByteArray): Boolean = noticeOutboxes[link]?.add(text) ?: false

    override fun receiveBufferBytes(link: Long): Int = if (inboxes.containsKey(link)) receiveBufferOf(link) else 0

    fun receiveBufferOf(handle: Long): Int = RECEIVE_BUFFER_BASE_BYTES + handle.toInt()

    override fun socketDrops(link: Long): Int =
        if (inboxes.containsKey(link)) socketDropCounts[link] ?: UNKNOWN_COUNT else UNKNOWN_COUNT

    fun dropOnSocket(handle: Long, count: Int) {
        socketDropCounts[handle] = count
    }

    override fun inboxDrops(link: Long): Int =
        if (inboxes.containsKey(link)) inboxDropCounts[link] ?: UNKNOWN_COUNT else UNKNOWN_COUNT

    fun dropInInbox(handle: Long, count: Int) {
        inboxDropCounts[handle] = count
    }

    override fun takeLongestPauseMs(link: Long): Int =
        if (inboxes.containsKey(link)) longestPauses.remove(link) ?: NO_PAUSE_MS else UNKNOWN_COUNT

    fun pauseFor(handle: Long, pauseMs: Int) {
        longestPauses[handle] = pauseMs
    }

    override fun requestKeyFrame(link: Long) {
        keyFrameRequests.add(link)
    }

    override fun close(link: Long) {
        closedLinks.add(link)
        inboxes.remove(link)
        statsOutboxes.remove(link)
        noticeOutboxes.remove(link)
    }

    private fun push(handle: Long, kind: Int, payload: ByteArray) {
        inboxes[handle]?.add(byteArrayOf(kind.toByte()) + payload)
    }

    private fun nextSent(outboxes: Map<Long, LinkedBlockingQueue<ByteArray>>, handle: Long): String {
        val outbox = outboxes[handle] ?: error("No open link $handle")
        val text = outbox.poll(SENT_TIMEOUT_MS, TimeUnit.MILLISECONDS) ?: error("Nothing was sent in time")
        return String(text, Charsets.UTF_8)
    }

    private companion object {
        const val NO_HANDLE = 0L
        const val RECEIVE_BUFFER_BASE_BYTES = 4_194_304
        const val UNKNOWN_COUNT = -1
        const val NO_PAUSE_MS = 0
        const val ACCEPT_TIMEOUT_MS = 5_000L
        const val SENT_TIMEOUT_MS = 5_000L
    }
}
