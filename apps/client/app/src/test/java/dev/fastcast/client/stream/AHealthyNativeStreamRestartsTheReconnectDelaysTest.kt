package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AHealthyNativeStreamRestartsTheReconnectDelaysTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient(
        { aStreamTarget() },
        listener,
        link,
        ReconnectBackoff(listOf(40L, 80L, 160L)),
        health = StreamHealth(HEALTHY_AFTER_MS),
    )

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `frames delivered across the healthy period restart the delays`() {
        client.start()
        link.end(link.accept(), "")
        val firstDelay = listener.nextRetryDelay()
        link.end(link.accept(), "")
        val secondDelay = listener.nextRetryDelay()

        val handle = link.accept()
        deliverFramesAcrossTheHealthyPeriod(handle)
        link.end(handle, "")
        val delayAfterHealthyStream = listener.nextRetryDelay()

        assertEquals(listOf(40L, 80L, 40L), listOf(firstDelay, secondDelay, delayAfterHealthyStream))
    }

    @Test
    fun `every connection has to prove itself again`() {
        client.start()
        val first = link.accept()
        deliverFramesAcrossTheHealthyPeriod(first)
        link.end(first, "")
        val delayAfterFirst = listener.nextRetryDelay()

        val second = link.accept()
        deliverFramesAcrossTheHealthyPeriod(second)
        link.end(second, "")
        val delayAfterSecond = listener.nextRetryDelay()

        assertEquals(listOf(40L, 40L), listOf(delayAfterFirst, delayAfterSecond))
    }

    @Test
    fun `a stream that flows for less than the healthy period keeps the delays growing`() {
        client.start()
        link.end(link.accept(), "")
        listener.nextRetryDelay()

        val handle = link.accept()
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1)))
        link.frame(handle, frameBytes(false, 1L, byteArrayOf(1)))
        repeat(2) { listener.nextFrame() }
        link.end(handle, "")

        assertEquals(80L, listener.nextRetryDelay())
    }

    @Test
    fun `the listener hears once that the stream proved healthy`() {
        client.start()
        val handle = link.acceptConnected()
        deliverFramesAcrossTheHealthyPeriod(handle)
        link.frame(handle, frameBytes(false, 2L, byteArrayOf(1)))
        listener.nextFrame()

        assertEquals(
            listOf(
                RecordingStreamListener.CONNECTING,
                RecordingStreamListener.CONNECTED,
                RecordingStreamListener.FRAME,
                RecordingStreamListener.HEALTHY,
                RecordingStreamListener.FRAME,
                RecordingStreamListener.FRAME,
            ),
            List(EVENTS_UNTIL_THIRD_FRAME) { listener.nextEvent() },
        )
        assertTrue(listener.hasNoEventWithin(NO_MORE_EVENTS_WAIT_MS))
    }

    private fun deliverFramesAcrossTheHealthyPeriod(handle: Long) {
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        Thread.sleep(HEALTHY_AFTER_MS + SLACK_MS)
        link.frame(handle, frameBytes(false, 1L, byteArrayOf(1)))
        listener.nextFrame()
    }

    private companion object {
        const val HEALTHY_AFTER_MS = 150L
        const val SLACK_MS = 100L
        const val EVENTS_UNTIL_THIRD_FRAME = 6
        const val NO_MORE_EVENTS_WAIT_MS = 200L
    }
}
