package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class StoppingTheNativeStreamClientEndsTheConversationForGoodTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `no new connection is attempted after the stop`() {
        client.start()
        assertNotNull(link.acceptWithin(5_000L))

        client.stop()

        assertNull(link.acceptWithin(400L))
    }

    @Test
    fun `no new connection is attempted when the stop follows a lost connection`() {
        client.start()
        link.end(link.accept(), "")
        listener.nextRetryDelay()

        client.stop()

        assertNull(link.acceptWithin(400L))
    }
}
