package dev.fastcast.client.discovery

import java.io.IOException
import java.net.DatagramPacket
import java.net.DatagramSocket
import java.net.Inet4Address
import java.net.InetAddress
import java.net.InetSocketAddress
import java.net.SocketTimeoutException
import java.util.concurrent.Executor
import java.util.concurrent.TimeUnit

class ServerFinder(
    private val listener: ServerFinderListener,
    private val deliver: Executor,
    private val targets: () -> List<InetAddress> = BroadcastTargets::find,
    private val port: Int = DiscoveryProtocol.PORT,
    private val probeIntervalMs: Long = PROBE_INTERVAL_MS,
    private val bindAddress: InetAddress? = null,
) {

    private var search: Search? = null

    @Synchronized
    fun start() {
        if (search != null) return
        search = Search().also(Search::begin)
    }

    @Synchronized
    fun stop() {
        search?.end()
        search = null
    }

    private inner class Search : Runnable {

        private val servers = LinkedHashMap<String, DiscoveredServer>()
        private val thread = Thread(this, THREAD_NAME).apply { isDaemon = true }

        @Volatile
        private var active = true

        @Volatile
        private var udp: DatagramSocket? = null

        fun begin() = thread.start()

        fun end() {
            active = false
            udp?.close()
            thread.join(JOIN_TIMEOUT_MS)
        }

        override fun run() {
            try {
                DatagramSocket(InetSocketAddress(bindAddress, 0)).use { socket ->
                    socket.broadcast = true
                    udp = socket
                    converse(socket)
                }
            } catch (e: IOException) {
                return
            }
        }

        private fun converse(socket: DatagramSocket) {
            val buffer = ByteArray(RECEIVE_BUFFER_BYTES)
            var nextProbeNs = System.nanoTime()
            while (active) {
                if (System.nanoTime() - nextProbeNs >= 0) {
                    sendProbes(socket)
                    nextProbeNs = System.nanoTime() + TimeUnit.MILLISECONDS.toNanos(probeIntervalMs)
                }
                socket.soTimeout = millisUntil(nextProbeNs)
                receive(socket, buffer)
            }
        }

        private fun sendProbes(socket: DatagramSocket) {
            val payload = DiscoveryProtocol.probe()
            targets().forEach { target -> send(socket, payload, target) }
        }

        private fun send(socket: DatagramSocket, payload: ByteArray, target: InetAddress) {
            try {
                socket.send(DatagramPacket(payload, payload.size, target, port))
            } catch (e: IOException) {
                return
            }
        }

        private fun millisUntil(deadlineNs: Long): Int =
            TimeUnit.NANOSECONDS.toMillis(deadlineNs - System.nanoTime()).coerceAtLeast(1L).toInt()

        private fun receive(socket: DatagramSocket, buffer: ByteArray) {
            val packet = DatagramPacket(buffer, buffer.size)
            try {
                socket.receive(packet)
            } catch (e: SocketTimeoutException) {
                return
            }
            accept(packet)
        }

        private fun accept(packet: DatagramPacket) {
            val source = packet.address as? Inet4Address ?: return
            val data = packet.data.copyOfRange(packet.offset, packet.offset + packet.length)
            val server = DiscoveryReply.parse(data, source.hostAddress ?: return) ?: return
            remember(server)
        }

        private fun remember(server: DiscoveredServer) {
            if (servers.put(server.address, server) == server) return
            val snapshot = servers.values.toList()
            deliver.execute { if (active) listener.onServersChanged(snapshot) }
        }
    }

    private companion object {
        const val THREAD_NAME = "fastcast-discovery"
        const val PROBE_INTERVAL_MS = 1_000L
        const val JOIN_TIMEOUT_MS = 1_000L
        const val RECEIVE_BUFFER_BYTES = DiscoveryProtocol.MAX_REPLY_BYTES + 1
    }
}
