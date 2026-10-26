package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNativeStreamClientAsksForTheTargetOnEveryConnectionTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val attempts = AtomicInteger()
    private val client = NativeStreamClient(
        { aStreamTarget(FIRST_PORT + attempts.getAndIncrement()) },
        listener,
        link,
        ReconnectBackoff(listOf(30L)),
    )

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `every connection carries the target as it is by then`() {
        client.start()
        link.accept()
        client.reconnect()
        val second = link.accept()
        link.end(second, "")
        link.accept()

        assertEquals(
            listOf(FIRST_PORT, FIRST_PORT + 1, FIRST_PORT + 2),
            link.targets.map(StreamTarget::port),
        )
    }

    private companion object {
        const val FIRST_PORT = 4000
    }
}
