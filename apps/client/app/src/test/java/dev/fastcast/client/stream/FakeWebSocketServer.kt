package dev.fastcast.client.stream

import java.net.InetAddress
import java.net.ServerSocket
import java.net.Socket
import java.net.SocketTimeoutException
import java.security.MessageDigest
import java.util.Base64

class FakeWebSocketServer : AutoCloseable {

    private val serverSocket = ServerSocket(0, BACKLOG, InetAddress.getLoopbackAddress())

    val address: String = "127.0.0.1:${serverSocket.localPort}"

    val url: String = "ws://$address/ws"

    fun accept(): FakeConnection = acceptWithin(ACCEPT_TIMEOUT_MS) ?: error("No client connected in time")

    fun acceptWithin(timeoutMs: Int): FakeConnection? {
        serverSocket.soTimeout = timeoutMs
        val socket = try {
            serverSocket.accept()
        } catch (e: SocketTimeoutException) {
            return null
        }
        val requestTarget = answerHandshake(socket)
        return FakeConnection(socket, requestTarget)
    }

    override fun close() = serverSocket.close()

    private fun answerHandshake(socket: Socket): String {
        val requestLines = readRequestLines(socket)
        val response = "HTTP/1.1 101 Switching Protocols\r\n" +
            "Upgrade: websocket\r\n" +
            "Connection: Upgrade\r\n" +
            "Sec-WebSocket-Accept: ${acceptValue(secWebSocketKey(requestLines))}\r\n\r\n"
        socket.getOutputStream().apply {
            write(response.toByteArray(Charsets.ISO_8859_1))
            flush()
        }
        return requestLines.first().split(REQUEST_LINE_SEPARATOR)[REQUEST_TARGET_INDEX]
    }

    private fun readRequestLines(socket: Socket): List<String> {
        val input = socket.getInputStream()
        val request = StringBuilder()
        while (!request.endsWith(HEADER_END)) {
            val next = input.read()
            check(next >= 0) { "Client closed during the handshake" }
            request.append(next.toChar())
        }
        return request.lines()
    }

    private fun secWebSocketKey(requestLines: List<String>): String = requestLines
        .firstNotNullOf { line -> line.takeIf { it.startsWith(KEY_HEADER, ignoreCase = true) } }
        .substringAfter(':')
        .trim()

    private fun acceptValue(key: String): String {
        val digest = MessageDigest.getInstance("SHA-1").digest((key + HANDSHAKE_GUID).toByteArray(Charsets.ISO_8859_1))
        return Base64.getEncoder().encodeToString(digest)
    }

    private companion object {
        const val BACKLOG = 10
        const val ACCEPT_TIMEOUT_MS = 5_000
        const val HEADER_END = "\r\n\r\n"
        const val KEY_HEADER = "Sec-WebSocket-Key"
        const val REQUEST_LINE_SEPARATOR = ' '
        const val REQUEST_TARGET_INDEX = 1
        const val HANDSHAKE_GUID = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11"
    }
}
