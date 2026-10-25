package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aConfigMessage
import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Test

class AnOverfullBacklogResumesFromTheNewestKeyframeTest {

    private val backlog = PacketBacklog(limit = 4)
    private val oldKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))
    private val oldDelta1 = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 16L))
    private val oldDelta2 = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 33L))
    private val oldDelta3 = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 50L))
    private val newKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 66L))
    private val newDelta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 83L))
    private val config = Packet.Config(aConfigMessage())

    private fun drain(): List<Packet> = generateSequence { backlog.next() }.toList()

    @Test
    fun `everything before the newest keyframe is discarded`() {
        listOf(oldKey, oldDelta1, oldDelta2, oldDelta3, newKey, newDelta).forEach(backlog::add)

        assertEquals(listOf(newKey, newDelta), drain())
    }

    @Test
    fun `the discarded frames are counted as dropped`() {
        listOf(oldKey, oldDelta1, oldDelta2, oldDelta3, newKey).forEach(backlog::add)

        assertEquals(4L, backlog.droppedFrames.get())
    }

    @Test
    fun `the config that precedes the newest keyframe is kept in front of it`() {
        listOf(config, oldKey, oldDelta1, oldDelta2, newKey, newDelta).forEach(backlog::add)

        assertEquals(listOf(config, newKey, newDelta), drain())
    }

    @Test
    fun `a long backlog collapses to a keyframe that arrives late`() {
        backlog.add(oldKey)
        repeat(100) { backlog.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = it.toLong()))) }
        backlog.add(newKey)

        assertEquals(listOf(newKey), drain())
        assertEquals(101L, backlog.droppedFrames.get())
    }
}
