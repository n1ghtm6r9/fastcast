package dev.fastcast.client.stream

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import java.util.Base64
import org.junit.After
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheNativeStreamClientDeliversTheConfigAndTheFramesInOrderTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link)
    private val description = Base64.getEncoder().encodeToString(RealAvcConfigurationRecord.BYTES)
    private val configJson =
        """{"type":"config","codec":"avc1.64002a","width":2560,"height":1440,"fps":60,"description":"$description"}"""

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `the client announces the connection before the server speaks`() {
        client.start()
        link.acceptConnected()

        assertEquals(RecordingStreamListener.CONNECTING, listener.nextEvent())
        assertEquals(RecordingStreamListener.CONNECTED, listener.nextEvent())
    }

    @Test
    fun `the config comes first and the frames follow in the order they were sent`() {
        client.start()
        val handle = link.acceptConnected()

        link.config(handle, configJson)
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1, 2, 3)))
        link.frame(handle, frameBytes(false, 16L, byteArrayOf(4, 5)))
        link.frame(handle, frameBytes(false, 33L, byteArrayOf(6)))

        val events = List(6) { listener.nextEvent() }
        assertEquals(
            listOf(
                RecordingStreamListener.CONNECTING,
                RecordingStreamListener.CONNECTED,
                RecordingStreamListener.CONFIG,
                RecordingStreamListener.FRAME,
                RecordingStreamListener.FRAME,
                RecordingStreamListener.FRAME,
            ),
            events,
        )
    }

    @Test
    fun `the delivered config carries the parsed description`() {
        client.start()
        link.config(link.accept(), configJson)

        val config = listener.nextConfig()

        assertEquals(2560, config.width)
        assertEquals(1440, config.height)
        assertArrayEquals(RealAvcConfigurationRecord.BYTES, config.description)
    }

    @Test
    fun `the delivered frames carry their flags timestamps and payloads`() {
        client.start()
        val handle = link.accept()

        link.frame(handle, frameBytes(true, 1_000L, byteArrayOf(1, 2, 3)))
        link.frame(handle, frameBytes(false, 1_016L, byteArrayOf(4, 5)))

        val first = listener.nextFrame()
        val second = listener.nextFrame()
        assertTrue(first.keyframe)
        assertEquals(1_000L, first.timestampMs)
        assertEquals(3, first.payloadSize)
        assertFalse(second.keyframe)
        assertEquals(1_016L, second.timestampMs)
        assertEquals(2, second.payloadSize)
    }

    @Test
    fun `a frame of two hundred thousand bytes is delivered whole`() {
        client.start()
        val payload = ByteArray(200_000) { (it % 251).toByte() }

        link.frame(link.accept(), frameBytes(true, 5L, payload))

        val frame = listener.nextFrame()
        assertEquals(payload.size, frame.payloadSize)
        assertEquals(payload[199_999], frame.data[frame.payloadOffset + 199_999])
    }

    @Test
    fun `the client counts every byte it receives`() {
        client.start()
        val handle = link.accept()
        val first = frameBytes(true, 0L, ByteArray(100))
        val second = frameBytes(false, 16L, ByteArray(40))

        link.frame(handle, first)
        link.frame(handle, second)
        listener.nextFrame()
        listener.nextFrame()

        assertEquals((first.size + second.size).toLong(), client.bytesReceived.get())
    }

    @Test
    fun `a text message that is not a config is ignored`() {
        client.start()
        val handle = link.acceptConnected()

        link.config(handle, "hello")
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1)))

        val events = List(3) { listener.nextEvent() }
        assertEquals(
            listOf(
                RecordingStreamListener.CONNECTING,
                RecordingStreamListener.CONNECTED,
                RecordingStreamListener.FRAME,
            ),
            events,
        )
    }
}
