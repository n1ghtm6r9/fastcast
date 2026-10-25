package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Test

class ABacklogWaitsForAKeyframeBeforeItDeliversFramesTest {

    private val backlog = PacketBacklog(limit = 30)

    @Test
    fun `a fresh backlog holds back delta frames`() {
        backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 0L)))

        assertNull(backlog.next())
        assertEquals(1L, backlog.droppedFrames.get())
    }

    @Test
    fun `the first keyframe is delivered and the delta frames after it follow`() {
        val delta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 1L))
        val key = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 2L))
        val next = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 3L))
        backlog.add(delta)
        backlog.add(key)
        backlog.add(next)

        assertSame(key, backlog.next())
        assertSame(next, backlog.next())
        assertNull(backlog.next())
        assertEquals(1L, backlog.droppedFrames.get())
    }

    @Test
    fun `an empty backlog delivers nothing`() {
        assertNull(backlog.next())
    }
}
