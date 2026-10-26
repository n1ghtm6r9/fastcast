package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import java.util.concurrent.TimeUnit
import org.junit.After
import org.junit.Assert.assertTrue
import org.junit.Test

class ASlowStartOfANativeConnectionIsNotMistakenForSilenceTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient(
        { aStreamTarget() },
        listener,
        link,
        ReconnectBackoff(listOf(30L)),
        SilenceTimer(FIRST_MESSAGE_LIMIT_MS, STREAMING_LIMIT_MS),
    )

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a connection waiting for the first message outlasts the streaming limit`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        assertTrue(listener.hasNoEventWithin(BEYOND_STREAMING_LIMIT_MS))
    }

    @Test
    fun `a connection that never gets a first message is still dropped`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        listener.skipTo(RecordingStreamListener.DISCONNECTED)
    }

    @Test
    fun `a stream that goes quiet is dropped at the streaming limit`() {
        client.start()
        val handle = link.accept()
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        val quietSince = System.nanoTime()

        listener.skipTo(RecordingStreamListener.DISCONNECTED)

        assertTrue(millisSince(quietSince) < WELL_BEFORE_THE_FIRST_MESSAGE_LIMIT_MS)
    }

    @Test
    fun `a reconnected connection is given the longer limit again`() {
        client.start()
        val first = link.accept()
        link.frame(first, frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        link.end(first, "")
        listener.skipTo(RecordingStreamListener.DISCONNECTED)
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        assertTrue(listener.hasNoEventWithin(BEYOND_STREAMING_LIMIT_MS))
    }

    private fun millisSince(startNs: Long): Long = TimeUnit.NANOSECONDS.toMillis(System.nanoTime() - startNs)

    private companion object {
        const val FIRST_MESSAGE_LIMIT_MS = 1_500L
        const val STREAMING_LIMIT_MS = 300L
        const val BEYOND_STREAMING_LIMIT_MS = 700L
        const val WELL_BEFORE_THE_FIRST_MESSAGE_LIMIT_MS = 1_000L
    }
}
