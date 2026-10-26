package dev.fastcast.client.decoder

import dev.fastcast.client.stream.ConfigMessage

object ConfigChange {

    fun needsNewDecoder(running: ConfigMessage?, rendered: Boolean, next: ConfigMessage): Boolean =
        running == null || !rendered || differs(running, next)

    private fun differs(running: ConfigMessage, next: ConfigMessage): Boolean =
        running.codec != next.codec ||
            running.width != next.width ||
            running.height != next.height ||
            running.fps != next.fps ||
            !running.description.contentEquals(next.description)
}
