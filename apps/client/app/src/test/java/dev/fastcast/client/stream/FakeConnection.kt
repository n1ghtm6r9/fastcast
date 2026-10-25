package dev.fastcast.client.stream

import java.io.ByteArrayOutputStream
import java.io.DataInputStream
import java.net.Socket

class FakeConnection(private val socket: Socket, val requestTarget: String) : AutoCloseable {

    private val output = socket.getOutputStream()
    private val input = DataInputStream(socket.getInputStream())

    init {
        socket.soTimeout = READ_TIMEOUT_MS
    }

    fun sendText(text: String) = send(OPCODE_TEXT, text.toByteArray(Charsets.UTF_8))

    fun sendBinary(bytes: ByteArray) = send(OPCODE_BINARY, bytes)

    fun nextText(): String {
        val opcode = input.readUnsignedByte() and OPCODE_MASK
        check(opcode == OPCODE_TEXT) { "Expected a text frame but got opcode $opcode" }
        val lengthByte = input.readUnsignedByte()
        check(lengthByte and MASK_BIT != 0) { "Client frames must be masked" }
        val length = readLength(lengthByte and LENGTH_MASK)
        val maskKey = ByteArray(MASK_KEY_SIZE).also(input::readFully)
        val payload = ByteArray(length).also(input::readFully)
        return String(unmask(payload, maskKey), Charsets.UTF_8)
    }

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

    private fun readLength(sevenBitLength: Int): Int = when (sevenBitLength) {
        SHORT_LENGTH_MARKER -> input.readUnsignedShort()
        LONG_LENGTH_MARKER -> error("Frames with a 64-bit length are not supported")
        else -> sevenBitLength
    }

    private fun unmask(payload: ByteArray, maskKey: ByteArray): ByteArray =
        ByteArray(payload.size) { index ->
            (payload[index].toInt() xor maskKey[index % MASK_KEY_SIZE].toInt()).toByte()
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
        const val OPCODE_MASK = 0x0F
        const val MASK_BIT = 0x80
        const val LENGTH_MASK = 0x7F
        const val MASK_KEY_SIZE = 4
        const val READ_TIMEOUT_MS = 5_000
        const val SHORT_LENGTH_LIMIT = 126
        const val SHORT_LENGTH_MARKER = 126
        const val LONG_LENGTH_MARKER = 127
        const val MAX_SHORT_EXTENDED_LENGTH = 0xFFFF
        const val BYTE_MASK = 0xFF
        val LONG_LENGTH_SHIFTS = 56 downTo 0 step Byte.SIZE_BITS
    }
}
