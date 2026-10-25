package dev.fastcast.client.stream

import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class AHealthyStreamRestartsTheReconnectDelaysTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val client = StreamClient(
        { server.url },
        listener,
        ReconnectBackoff(listOf(40L, 80L, 160L)),
        health = StreamHealth(HEALTHY_AFTER_MS),
    )

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `frames delivered across the healthy period restart the delays`() {
        client.start()
        server.accept().drop()
        val firstDelay = listener.nextRetryDelay()
        server.accept().drop()
        val secondDelay = listener.nextRetryDelay()

        val connection = server.accept()
        deliverFramesAcrossTheHealthyPeriod(connection)
        connection.drop()
        val delayAfterHealthyStream = listener.nextRetryDelay()

        assertEquals(listOf(40L, 80L, 40L), listOf(firstDelay, secondDelay, delayAfterHealthyStream))
    }

    @Test
    fun `every connection has to prove itself again`() {
        client.start()
        val first = server.accept()
        deliverFramesAcrossTheHealthyPeriod(first)
        first.drop()
        val delayAfterFirst = listener.nextRetryDelay()

        val second = server.accept()
        deliverFramesAcrossTheHealthyPeriod(second)
        second.drop()
        val delayAfterSecond = listener.nextRetryDelay()

        assertEquals(listOf(40L, 40L), listOf(delayAfterFirst, delayAfterSecond))
    }

    @Test
    fun `a stream that flows for less than the healthy period keeps the delays growing`() {
        client.start()
        server.accept().drop()
        listener.nextRetryDelay()

        val connection = server.accept()
        connection.sendBinary(frameBytes(true, 0L, byteArrayOf(1)))
        connection.sendBinary(frameBytes(false, 1L, byteArrayOf(1)))
        repeat(2) { listener.nextFrame() }
        connection.drop()

        assertEquals(80L, listener.nextRetryDelay())
    }

    @Test
    fun `the listener hears once that the stream proved healthy`() {
        client.start()
        val connection = server.accept()
        deliverFramesAcrossTheHealthyPeriod(connection)
        connection.sendBinary(frameBytes(false, 2L, byteArrayOf(1)))
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

    private fun deliverFramesAcrossTheHealthyPeriod(connection: FakeConnection) {
        connection.sendBinary(frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        Thread.sleep(HEALTHY_AFTER_MS + SLACK_MS)
        connection.sendBinary(frameBytes(false, 1L, byteArrayOf(1)))
        listener.nextFrame()
    }

    private companion object {
        const val HEALTHY_AFTER_MS = 150L
        const val SLACK_MS = 100L
        const val EVENTS_UNTIL_THIRD_FRAME = 6
        const val NO_MORE_EVENTS_WAIT_MS = 200L
    }
}
