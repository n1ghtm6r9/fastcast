package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNativeStreamClientReconnectsWithGrowingDelaysTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient(
        { aStreamTarget() },
        listener,
        link,
        ReconnectBackoff(listOf(40L, 80L, 160L)),
    )

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `each lost connection is followed by a longer wait until the cap`() {
        client.start()

        val delays = List(4) {
            link.end(link.accept(), "")
            listener.nextRetryDelay()
        }

        assertEquals(listOf(40L, 80L, 160L, 160L), delays)
    }

    @Test
    fun `a delivered frame alone does not restart the delays`() {
        client.start()
        link.end(link.accept(), "")
        val firstDelay = listener.nextRetryDelay()
        link.end(link.accept(), "")
        val secondDelay = listener.nextRetryDelay()

        val handle = link.accept()
        link.frame(handle, frameBytes(true, 0L, byteArrayOf(1)))
        listener.nextFrame()
        link.end(handle, "")
        val delayAfterFrame = listener.nextRetryDelay()

        assertEquals(listOf(40L, 80L, 160L), listOf(firstDelay, secondDelay, delayAfterFrame))
    }

    @Test
    fun `every new attempt is announced as connecting`() {
        client.start()
        link.end(link.accept(), "")
        link.end(link.accept(), "")
        link.accept()

        repeat(3) { listener.skipTo(RecordingStreamListener.CONNECTING) }
    }
}
