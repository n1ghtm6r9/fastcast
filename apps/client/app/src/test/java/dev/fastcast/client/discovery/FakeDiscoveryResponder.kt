package dev.fastcast.client.discovery

import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.InetAddress
import java.util.concurrent.LinkedBlockingQueue
import java.util.concurrent.TimeUnit

class FakeDiscoveryResponder : AutoCloseable {

    private val socket = DatagramSocket(0, LOOPBACK)
    private val probes = LinkedBlockingQueue<String>()

    @Volatile
    private var replies: List<ByteArray> = emptyList()

    private val serving = Thread(::serve, THREAD_NAME).apply {
        isDaemon = true
        start()
    }

    val address: InetAddress = LOOPBACK
    val port: Int = socket.localPort

    fun answerWith(vararg texts: String) {
        replies = texts.map { it.toByteArray(Charsets.UTF_8) }
    }

    fun answerWithBytes(vararg payloads: ByteArray) {
        replies = payloads.toList()
    }

    fun nextProbe(): String =
        probes.poll(TIMEOUT_SECONDS, TimeUnit.SECONDS) ?: error("Timed out waiting for a probe")

    fun hasNoProbeWithin(timeoutMs: Long): Boolean = probes.poll(timeoutMs, TimeUnit.MILLISECONDS) == null

    fun discardProbesUntilQuiet(quietMs: Long) {
        repeat(MAX_DISCARDS) {
            if (probes.poll(quietMs, TimeUnit.MILLISECONDS) == null) return
        }
    }

    override fun close() {
        socket.close()
        serving.join(JOIN_TIMEOUT_MS)
    }

    private fun serve() {
        val buffer = ByteArray(BUFFER_BYTES)
        while (!socket.isClosed) answerOneProbe(buffer)
    }

    private fun answerOneProbe(buffer: ByteArray) {
        val probe = DatagramPacket(buffer, buffer.size)
        try {
            socket.receive(probe)
            probes.add(String(probe.data, probe.offset, probe.length, Charsets.UTF_8))
            replies.forEach { reply -> socket.send(DatagramPacket(reply, reply.size, probe.socketAddress)) }
        } catch (e: IOException) {
            return
        }
    }

    private companion object {
        val LOOPBACK: InetAddress = InetAddress.getByAddress(byteArrayOf(127, 0, 0, 1))
        const val THREAD_NAME = "fake-discovery-responder"
        const val BUFFER_BYTES = 2048
        const val TIMEOUT_SECONDS = 5L
        const val JOIN_TIMEOUT_MS = 2_000L
        const val MAX_DISCARDS = 50
    }
}
