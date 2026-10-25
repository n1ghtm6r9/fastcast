package dev.fastcast.client.decoder

import java.util.concurrent.atomic.AtomicLong

class PacketBacklog(
    private val limit: Int,
    val droppedFrames: AtomicLong = AtomicLong(),
) {

    private val packets = ArrayDeque<Packet>()
    private var awaitingKeyframe = true

    val isEmpty: Boolean
        get() = packets.isEmpty()

    fun add(packet: Packet) {
        packets.addLast(packet)
        if (packets.size > limit) trim()
    }

    fun next(): Packet? {
        while (true) {
            val packet = packets.removeFirstOrNull() ?: return null
            if (accepts(packet)) return packet
            droppedFrames.incrementAndGet()
        }
    }

    fun awaitKeyframe() {
        awaitingKeyframe = true
    }

    fun clear() {
        replaceWith(emptyList())
        awaitingKeyframe = true
    }

    private fun accepts(packet: Packet): Boolean = when (packet) {
        is Packet.Config -> {
            awaitingKeyframe = true
            true
        }
        is Packet.Frame -> acceptsFrame(packet)
    }

    private fun acceptsFrame(packet: Packet.Frame): Boolean {
        if (packet.frame.keyframe) {
            awaitingKeyframe = false
            return true
        }
        return !awaitingKeyframe
    }

    private fun trim() {
        val keyframeIndex = packets.indexOfLast(::isKeyframe)
        val configIndex = packets.indexOfLast { it is Packet.Config }
        when {
            configIndex > keyframeIndex -> replaceWith(listOf(packets[configIndex]))
            keyframeIndex >= 0 -> replaceWith(
                listOfNotNull(packets.getOrNull(configIndex)) + packets.subList(keyframeIndex, packets.size),
            )
            else -> clear()
        }
    }

    private fun replaceWith(kept: List<Packet>) {
        if (kept.size == packets.size) return
        val dropped = packets.count { it is Packet.Frame } - kept.count { it is Packet.Frame }
        droppedFrames.addAndGet(dropped.toLong())
        packets.clear()
        packets.addAll(kept)
    }

    private fun isKeyframe(packet: Packet): Boolean = packet is Packet.Frame && packet.frame.keyframe
}
