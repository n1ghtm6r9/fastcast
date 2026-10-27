package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Test

class TheNativeStreamClientTellsTheListenerToDiagnoseTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a diagnose notice reaches the listener with its run`() {
        client.start()
        val handle = link.acceptConnected()

        link.notice(handle, """{"type":"diagnose","run":42}""")

        assertEquals(42L, listener.nextDiagnosis())
    }

    @Test
    fun `a scale notice is not taken for a diagnose one`() {
        client.start()
        val handle = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        link.notice(handle, """{"type":"scale","percent":125,"min":100,"max":250}""")
        link.notice(handle, """{"type":"diagnose","run":7}""")

        assertEquals(RecordingStreamListener.SCALE, listener.nextEvent())
        assertEquals(RecordingStreamListener.DIAGNOSE, listener.nextEvent())
        assertEquals(7L, listener.nextDiagnosis())
    }
}
