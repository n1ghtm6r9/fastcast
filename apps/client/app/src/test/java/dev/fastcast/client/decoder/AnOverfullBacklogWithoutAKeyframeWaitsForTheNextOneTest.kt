package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Before
import org.junit.Test

class AnOverfullBacklogWithoutAKeyframeWaitsForTheNextOneTest {

    private val backlog = PacketBacklog(limit = 3)
    private val key = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))

    @Before
    fun deliverTheFirstKeyframe() {
        backlog.add(key)
        assertSame(key, backlog.next())
    }

    private fun addDeltas(count: Int) {
        repeat(count) { backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = it.toLong()))) }
    }

    @Test
    fun `the backlog is emptied when it overflows with delta frames only`() {
        addDeltas(4)

        assertEquals(true, backlog.isEmpty)
    }

    @Test
    fun `all the discarded delta frames are counted as dropped`() {
        addDeltas(4)

        assertEquals(4L, backlog.droppedFrames.get())
    }

    @Test
    fun `delta frames that arrive after the overflow are held back`() {
        addDeltas(4)
        backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 99L)))

        assertNull(backlog.next())
    }

    @Test
    fun `the next keyframe is delivered after the overflow`() {
        addDeltas(4)
        val nextKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 100L))
        backlog.add(nextKey)

        assertSame(nextKey, backlog.next())
    }
}
