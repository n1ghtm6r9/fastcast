package dev.fastcast.client.decoder

import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.stream.FrameMessage

sealed interface Packet {

    class Config(val config: ConfigMessage) : Packet

    class Frame(val frame: FrameMessage) : Packet
}
