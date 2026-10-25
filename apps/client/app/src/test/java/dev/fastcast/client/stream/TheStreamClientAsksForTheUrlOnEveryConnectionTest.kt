package dev.fastcast.client.stream

import java.util.concurrent.atomic.AtomicInteger
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheStreamClientAsksForTheUrlOnEveryConnectionTest {

    private val server = FakeWebSocketServer()
    private val listener = RecordingStreamListener()
    private val attempts = AtomicInteger()
    private val client = StreamClient(
        { "${server.url}?attempt=${attempts.incrementAndGet()}" },
        listener,
        ReconnectBackoff(listOf(30L)),
    )

    @After
    fun tearDown() {
        client.stop()
        server.close()
    }

    @Test
    fun `every connection carries the url as it is by then`() {
        client.start()
        val first = server.accept()
        client.reconnect()
        val second = server.accept()
        second.drop()
        val third = server.accept()

        assertEquals(
            listOf("/ws?attempt=1", "/ws?attempt=2", "/ws?attempt=3"),
            listOf(first.requestTarget, second.requestTarget, third.requestTarget),
        )
    }
}
