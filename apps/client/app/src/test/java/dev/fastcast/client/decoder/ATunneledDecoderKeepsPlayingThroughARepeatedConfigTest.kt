package dev.fastcast.client.decoder

import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.testing.aConfigMessage
import org.junit.Assert.assertEquals
import org.junit.Test

class ATunneledDecoderKeepsPlayingThroughARepeatedConfigTest {

    private val running = aConfigMessage()
    private val repeat = ConfigMessage(running.codec, running.width, running.height, running.fps, running.description.copyOf())

    @Test
    fun `a repeated config leaves a tunneled decoder running untouched`() {
        assertEquals(ConfigAction.KEEP, ConfigChange.actionFor(running, rendered = true, next = repeat, tunneled = true))
    }

    @Test
    fun `a tunneled decoder that has queued nothing yet keeps running through a repeat`() {
        assertEquals(ConfigAction.KEEP, ConfigChange.actionFor(running, rendered = false, next = repeat, tunneled = true))
    }

    @Test
    fun `a repeated config flushes a decoder that draws on the surface`() {
        assertEquals(ConfigAction.FLUSH, ConfigChange.actionFor(running, rendered = true, next = repeat, tunneled = false))
    }

    @Test
    fun `a surface decoder that has drawn nothing yet restarts on a repeat`() {
        assertEquals(ConfigAction.RESTART, ConfigChange.actionFor(running, rendered = false, next = repeat, tunneled = false))
    }

    @Test
    fun `a changed config restarts a tunneled decoder`() {
        val bigger = ConfigMessage(running.codec, 2560, 1440, running.fps, running.description)

        assertEquals(ConfigAction.RESTART, ConfigChange.actionFor(running, rendered = true, next = bigger, tunneled = true))
    }

    @Test
    fun `the first config starts a tunneled decoder`() {
        assertEquals(ConfigAction.RESTART, ConfigChange.actionFor(null, rendered = false, next = running, tunneled = true))
    }
}
