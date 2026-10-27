package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNativeStreamClientTellsTheListenerTheScreenScaleTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a scale notice reaches the listener with its percent and its limits`() {
        client.start()
        val handle = link.acceptConnected()

        link.notice(handle, """{"type":"scale","percent":175,"min":100,"max":300}""")

        val scale = listener.nextScale()
        assertEquals(175, scale.percent)
        assertEquals(100, scale.min)
        assertEquals(300, scale.max)
    }

    @Test
    fun `a notice that is not a scale is skipped`() {
        client.start()
        val handle = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        link.notice(handle, """{"type":"other","percent":150,"min":100,"max":300}""")
        link.notice(handle, """{"type":"scale","percent":125,"min":100,"max":250}""")

        assertEquals(RecordingStreamListener.SCALE, listener.nextEvent())
        assertEquals(125, listener.nextScale().percent)
    }
}
