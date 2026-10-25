package dev.fastcast.client.stream

import java.net.InetAddress
import java.net.ServerSocket
import java.net.Socket
import java.net.SocketTimeoutException
import java.security.MessageDigest
import java.util.Base64

class FakeWebSocketServer : AutoCloseable {

    private val serverSocket = ServerSocket(0, BACKLOG, InetAddress.getLoopbackAddress())

    val url: String = "ws://127.0.0.1:${serverSocket.localPort}/ws"

    fun accept(): FakeConnection = acceptWithin(ACCEPT_TIMEOUT_MS) ?: error("No client connected in time")

    fun acceptWithin(timeoutMs: Int): FakeConnection? {
        serverSocket.soTimeout = timeoutMs
        val socket = try {
            serverSocket.accept()
        } catch (e: SocketTimeoutException) {
            return null
        }
        answerHandshake(socket)
        return FakeConnection(socket)
    }

    override fun close() = serverSocket.close()

    private fun answerHandshake(socket: Socket) {
        val key = readSecWebSocketKey(socket)
        val response = "HTTP/1.1 101 Switching Protocols\r\n" +
            "Upgrade: websocket\r\n" +
            "Connection: Upgrade\r\n" +
            "Sec-WebSocket-Accept: ${acceptValue(key)}\r\n\r\n"
        socket.getOutputStream().apply {
            write(response.toByteArray(Charsets.ISO_8859_1))
            flush()
        }
    }

    private fun readSecWebSocketKey(socket: Socket): String {
        val input = socket.getInputStream()
        val request = StringBuilder()
        while (!request.endsWith(HEADER_END)) {
            val next = input.read()
            check(next >= 0) { "Client closed during the handshake" }
            request.append(next.toChar())
        }
        return request.lines()
            .firstNotNullOf { line -> line.takeIf { it.startsWith(KEY_HEADER, ignoreCase = true) } }
            .substringAfter(':')
            .trim()
    }

    private fun acceptValue(key: String): String {
        val digest = MessageDigest.getInstance("SHA-1").digest((key + HANDSHAKE_GUID).toByteArray(Charsets.ISO_8859_1))
        return Base64.getEncoder().encodeToString(digest)
    }

    private companion object {
        const val BACKLOG = 10
        const val ACCEPT_TIMEOUT_MS = 5_000
        const val HEADER_END = "\r\n\r\n"
        const val KEY_HEADER = "Sec-WebSocket-Key"
        const val HANDSHAKE_GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
    }
}
