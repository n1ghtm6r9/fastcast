package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class AnUnopenableLinkIsRetriedWithGrowingDelaysTest {

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
    fun `each refused open is followed by a longer wait and then a connection`() {
        link.refuseNextOpens(2)

        client.start()
        val delays = List(2) { listener.nextRetryDelay() }
        val handle = link.accept()

        assertEquals(listOf(40L, 80L), delays)
        assertEquals(1L, handle)
    }

    @Test
    fun `a refused open is reported as a lost connection with its reason`() {
        link.refuseNextOpens(1)

        client.start()

        assertEquals("Could not open the stream", listener.nextReason())
    }

    @Test
    fun `every attempt is announced as connecting`() {
        link.refuseNextOpens(2)

        client.start()
        link.accept()

        repeat(3) { listener.skipTo(RecordingStreamListener.CONNECTING) }
    }

    @Test
    fun `every attempt asks the link for the target`() {
        link.refuseNextOpens(2)

        client.start()
        link.accept()

        assertEquals(3, link.targets.size)
    }
}
