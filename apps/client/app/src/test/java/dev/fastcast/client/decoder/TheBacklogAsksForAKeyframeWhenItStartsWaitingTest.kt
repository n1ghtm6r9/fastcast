package dev.fastcast.client.decoder

import dev.fastcast.client.testing.aConfigMessage
import dev.fastcast.client.testing.aFrameMessage
import org.junit.Assert.assertEquals
import org.junit.Assert.assertSame
import org.junit.Before
import org.junit.Test

class TheBacklogAsksForAKeyframeWhenItStartsWaitingTest {

    private var requests = 0
    private val backlog = PacketBacklog(limit = 3, onKeyframeNeeded = { requests++ })
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
    fun `a backlog that has not lost anything has asked for nothing`() {
        assertEquals(0, requests)
    }

    @Test
    fun `the initial wait for the first keyframe asks for nothing`() {
        val fresh = PacketBacklog(limit = 3, onKeyframeNeeded = { requests++ })

        fresh.add(Packet.Frame(aFrameMessage(keyframe = false, timestampMs = 0L)))
        fresh.next()

        assertEquals(0, requests)
    }

    @Test
    fun `awaiting a keyframe asks once`() {
        backlog.awaitKeyframe()

        assertEquals(1, requests)
    }

    @Test
    fun `clearing the backlog asks once`() {
        backlog.clear()

        assertEquals(1, requests)
    }

    @Test
    fun `an overflow of delta frames asks once`() {
        addDeltas(4)

        assertEquals(1, requests)
    }

    @Test
    fun `an overflow that keeps a keyframe asks for nothing`() {
        addDeltas(2)
        backlog.add(Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 50L)))
        addDeltas(1)

        assertEquals(0, requests)
    }

    @Test
    fun `an overflow that keeps a config asks for nothing`() {
        addDeltas(2)
        backlog.add(Packet.Config(aConfigMessage()))
        addDeltas(1)

        assertEquals(0, requests)
    }

    @Test
    fun `a config asks for nothing`() {
        backlog.add(Packet.Config(aConfigMessage()))
        backlog.next()

        assertEquals(0, requests)
    }

    @Test
    fun `waiting again while already waiting does not ask again`() {
        backlog.awaitKeyframe()
        backlog.awaitKeyframe()
        backlog.clear()

        assertEquals(1, requests)
    }

    @Test
    fun `a keyframe ends the wait and the next one asks again`() {
        backlog.awaitKeyframe()
        val next = Packet.Frame(aFrameMessage(keyframe = true, timestampMs = 33L))
        backlog.add(next)
        assertSame(next, backlog.next())

        backlog.awaitKeyframe()

        assertEquals(2, requests)
    }
}
