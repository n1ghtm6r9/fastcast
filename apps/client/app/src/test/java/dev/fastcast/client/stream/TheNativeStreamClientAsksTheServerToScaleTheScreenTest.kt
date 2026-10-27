package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import org.json.JSONObject
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheNativeStreamClientAsksTheServerToScaleTheScreenTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient({ aStreamTarget() }, listener, link, ReconnectBackoff(listOf(30L)))

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `an ask reaches the server as a scale notice with its step`() {
        client.start()
        val handle = link.acceptConnected()

        assertTrue(client.askScale(-1))

        val ask = JSONObject(link.nextNotice(handle))
        assertEquals("scale", ask.getString("type"))
        assertEquals(-1, ask.getInt("step"))
    }

    @Test
    fun `nothing is asked before the client is started`() {
        assertFalse(client.askScale(1))
    }
}
