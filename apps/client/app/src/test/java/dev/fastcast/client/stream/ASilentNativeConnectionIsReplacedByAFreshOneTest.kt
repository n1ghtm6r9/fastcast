package dev.fastcast.client.stream

import dev.fastcast.client.testing.aStreamTarget
import dev.fastcast.client.testing.frameBytes
import org.junit.After
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class ASilentNativeConnectionIsReplacedByAFreshOneTest {

    private val link = FakeStreamLink()
    private val listener = RecordingStreamListener()
    private val client = NativeStreamClient(
        { aStreamTarget() },
        listener,
        link,
        ReconnectBackoff(listOf(30L)),
        SilenceTimer(SILENCE_LIMIT_MS, SILENCE_LIMIT_MS),
    )

    @After
    fun tearDown() {
        client.stop()
    }

    @Test
    fun `a connection that stays silent past the limit is reopened`() {
        client.start()
        link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        listener.skipTo(RecordingStreamListener.DISCONNECTED)
        link.frame(link.accept(), frameBytes(true, 9L, byteArrayOf(1)))

        assertEquals(9L, listener.nextFrame().timestampMs)
    }

    @Test
    fun `a connection that keeps delivering frames is kept`() {
        client.start()
        val handle = link.acceptConnected()
        listener.skipTo(RecordingStreamListener.CONNECTED)

        repeat(FRAMES) { index ->
            link.frame(handle, frameBytes(index == 0, index.toLong(), byteArrayOf(1)))
            Thread.sleep(FRAME_GAP_MS)
        }

        assertNull(link.acceptWithin(NO_RECONNECT_WAIT_MS))
    }

    private companion object {
        const val SILENCE_LIMIT_MS = 300L
        const val FRAMES = 20
        const val FRAME_GAP_MS = 50L
        const val NO_RECONNECT_WAIT_MS = 200L
    }
}
