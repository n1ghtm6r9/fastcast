package dev.fastcast.client.decoder

import dev.fastcast.client.stream.ConfigMessage

object ConfigChange {

    fun actionFor(running: ConfigMessage?, rendered: Boolean, next: ConfigMessage, tunneled: Boolean): ConfigAction = when {
        tunneled && running != null && !differs(running, next) -> ConfigAction.KEEP
        needsNewDecoder(running, rendered, next) -> ConfigAction.RESTART
        else -> ConfigAction.FLUSH
    }

    fun needsNewDecoder(running: ConfigMessage?, rendered: Boolean, next: ConfigMessage): Boolean =
        running == null || !rendered || differs(running, next)

    private fun differs(running: ConfigMessage, next: ConfigMessage): Boolean =
        running.codec != next.codec || running.width != next.width || running.height != next.height ||
            running.fps != next.fps || !running.description.contentEquals(next.description)
}
