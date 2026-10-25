package dev.fastcast.client.stream

import java.io.ByteArrayOutputStream
import java.net.Socket

class FakeConnection(private val socket: Socket) : AutoCloseable {

    private val output = socket.getOutputStream()

    fun sendText(text: String) = send(OPCODE_TEXT, text.toByteArray(Charsets.UTF_8))

    fun sendBinary(bytes: ByteArray) = send(OPCODE_BINARY, bytes)

    fun drop() = socket.close()

    override fun close() = socket.close()

    private fun send(opcode: Int, payload: ByteArray) {
        output.write(encode(opcode, payload))
        output.flush()
    }

    private fun encode(opcode: Int, payload: ByteArray): ByteArray {
        val frame = ByteArrayOutputStream()
        frame.write(FINAL_FRAME_BIT or opcode)
        writeLength(frame, payload.size)
        frame.write(payload)
        return frame.toByteArray()
    }

    private fun writeLength(frame: ByteArrayOutputStream, length: Int) {
        when {
            length < SHORT_LENGTH_LIMIT -> frame.write(length)
            length <= MAX_SHORT_EXTENDED_LENGTH -> {
                frame.write(SHORT_LENGTH_MARKER)
                frame.write(length shr Byte.SIZE_BITS)
                frame.write(length and BYTE_MASK)
            }
            else -> {
                frame.write(LONG_LENGTH_MARKER)
                for (shift in LONG_LENGTH_SHIFTS) frame.write(((length.toLong() shr shift) and BYTE_MASK.toLong()).toInt())
            }
        }
    }

    private companion object {
        const val OPCODE_TEXT = 0x1
        const val OPCODE_BINARY = 0x2
        const val FINAL_FRAME_BIT = 0x80
        const val SHORT_LENGTH_LIMIT = 126
        const val SHORT_LENGTH_MARKER = 126
        const val LONG_LENGTH_MARKER = 127
        const val MAX_SHORT_EXTENDED_LENGTH = 0xFFFF
        const val BYTE_MASK = 0xFF
        val LONG_LENGTH_SHIFTS = 56 downTo 0 step Byte.SIZE_BITS
    }
}
