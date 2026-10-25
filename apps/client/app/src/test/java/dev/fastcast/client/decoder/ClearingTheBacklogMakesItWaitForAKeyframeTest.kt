package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Assert.assertTrue
import org.junit.Before
import org.junit.Test

class ClearingTheBacklogMakesItWaitForAKeyframeTest {

    private val backlog = PacketBacklog(limit = 30)
    private val key = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))

    @Before
    fun deliverTheFirstKeyframe() {
        backlog.add(key)
        assertSame(key, backlog.next())
    }

    @Test
    fun `clearing removes every queued packet`() {
        repeat(5) { backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = it.toLong()))) }

        backlog.clear()

        assertTrue(backlog.isEmpty)
    }

    @Test
    fun `the cleared frames are counted as dropped`() {
        repeat(5) { backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = it.toLong()))) }

        backlog.clear()

        assertEquals(5L, backlog.droppedFrames.get())
    }

    @Test
    fun `delta frames are held back until a keyframe arrives`() {
        backlog.clear()
        backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 16L)))

        assertNull(backlog.next())
    }

    @Test
    fun `awaiting a keyframe without clearing also holds back delta frames`() {
        backlog.awaitKeyframe()
        backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 16L)))
        val nextKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 33L))
        backlog.add(nextKey)

        assertSame(nextKey, backlog.next())
    }
}
