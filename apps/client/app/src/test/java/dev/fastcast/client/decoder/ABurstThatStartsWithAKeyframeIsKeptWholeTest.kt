package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aConfigMessage
import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Test

class ABurstThatStartsWithAKeyframeIsKeptWholeTest {

    private val backlog = PacketBacklog(limit = 30)
    private val config = Packet.Config(aConfigMessage())
    private val key = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 0L))
    private val deltas = List(60) { Packet.Frame(aFrameMessage(keyframe = false, timestampMs = (it + 1) * 16L)) }

    @Test
    fun `the connect burst is delivered in full and in order`() {
        backlog.add(config)
        backlog.add(key)
        deltas.forEach(backlog::add)

        val delivered = generateSequence { backlog.next() }.toList()

        assertEquals(listOf<Packet>(config, key) + deltas, delivered)
    }

    @Test
    fun `nothing in the connect burst counts as dropped`() {
        backlog.add(config)
        backlog.add(key)
        deltas.forEach(backlog::add)

        generateSequence { backlog.next() }.count()

        assertEquals(0L, backlog.droppedFrames.get())
    }
}
