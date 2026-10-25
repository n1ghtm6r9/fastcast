package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aConfigMessage
import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Assert.assertSame
import org.junit.Test

class AConfigMakesTheBacklogWaitForAKeyframeTest {

    private val backlog = PacketBacklog(limit = 30)
    private val firstKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))
    private val delta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 16L))
    private val config = Packet.Config(aConfigMessage())
    private val staleDelta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 33L))
    private val secondKey = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 50L))
    private val freshDelta = Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 66L))

    @Test
    fun `frames between a config and the next keyframe are dropped`() {
        listOf(firstKey, delta, config, staleDelta, secondKey, freshDelta).forEach(backlog::add)

        val delivered = generateSequence { backlog.next() }.toList()

        assertEquals(listOf(firstKey, delta, config, secondKey, freshDelta), delivered)
        assertEquals(1L, backlog.droppedFrames.get())
    }

    @Test
    fun `the config itself is always delivered`() {
        backlog.add(config)

        assertSame(config, backlog.next())
    }

    @Test
    fun `without a following keyframe nothing else is delivered`() {
        listOf(config, staleDelta).forEach(backlog::add)

        assertSame(config, backlog.next())
        assertNull(backlog.next())
    }
}
