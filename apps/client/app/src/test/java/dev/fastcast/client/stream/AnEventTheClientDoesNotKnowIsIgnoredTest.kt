package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AnEventTheClientDoesNotKnowIsIgnoredTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link)

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `an event of an unknown kind is skipped`() {
        client.start()
        val handle = link.accept()

        link.raw(handle, byteArrayOf(UNKNOWN_KIND, 1, 2, 3))
        link.frame(handle, frameBytes(true, 4L, byteArrayOf(1)))

        assertEquals(4L, listener.nextFrame().timestampMs)
    }

    @Test
    fun `an event without a kind is skipped`() {
        client.start()
        val handle = link.accept()

        link.raw(handle, ByteArray(0))
        link.frame(handle, frameBytes(true, 5L, byteArrayOf(1)))

        assertEquals(5L, listener.nextFrame().timestampMs)
    }

    @Test
    fun `a frame without a payload is not delivered`() {
        client.start()
        val handle = link.acceptConnected()

        link.frame(handle, ByteArray(FrameMessage.HEADER_SIZE))
        link.frame(handle, frameBytes(true, 6L, byteArrayOf(1)))

        assertEquals(6L, listener.nextFrame().timestampMs)
    }

    private companion object {
        const val UNKNOWN_KIND: Byte = 9
    }
}
