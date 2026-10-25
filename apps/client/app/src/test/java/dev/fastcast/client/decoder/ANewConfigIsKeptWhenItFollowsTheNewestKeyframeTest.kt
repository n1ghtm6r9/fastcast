package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aConfigMessage
import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Test

class ANewConfigIsKeptWhenItFollowsTheNewestKeyframeTest {

    private val backlog = PacketBacklog(limit = 3)
    private val oldKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))
    private val oldDelta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 16L))
    private val config = Packet.Config(aConfigMessage(width = 1920, height = 1080))
    private val lateDelta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 33L))
    private val newKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 50L))

    private fun drain(): List<Packet> = generateSequence { backlog.next() }.toList()

    @Test
    fun `an overflow keeps only the config and drops the frames before it`() {
        listOf(oldKey, oldDelta, config, lateDelta).forEach(backlog::add)

        assertEquals(listOf(config), drain())
    }

    @Test
    fun `the frames of the old configuration are counted as dropped`() {
        listOf(oldKey, oldDelta, config, lateDelta).forEach(backlog::add)

        assertEquals(3L, backlog.droppedFrames.get())
    }

    @Test
    fun `the first keyframe after the kept config is delivered`() {
        listOf(oldKey, oldDelta, config, lateDelta).forEach(backlog::add)
        backlog.add(newKey)

        assertEquals(listOf(config, newKey), drain())
    }
}
