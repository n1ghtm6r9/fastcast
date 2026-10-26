package dev.fastcast.client.decoder

import dev.fastcast.client.stream.ConfigMessage
import dev.fastcast.client.testing.aConfigMessage
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class ARenderingDecoderIsKeptForARepeatedConfigTest {

    private val running = aConfigMessage()
    private val repeat = ConfigMessage(running.codec, running.width, running.height, running.fps, running.description.copyOf())

    @Test
    fun `a repeated config keeps the decoder`() {
        assertFalse(ConfigChange.needsNewDecoder(running, rendered = true, next = repeat))
    }

    @Test
    fun `the first config needs a decoder`() {
        assertTrue(ConfigChange.needsNewDecoder(null, rendered = false, next = running))
    }

    @Test
    fun `a repeat before the first rendered frame needs a new decoder`() {
        assertTrue(ConfigChange.needsNewDecoder(running, rendered = false, next = repeat))
    }

    @Test
    fun `a change of any field needs a new decoder`() {
        val changedDescription = running.description.copyOf().also { it[0] = (it[0] + 1).toByte() }
        val changes = mapOf(
            "codec" to ConfigMessage("avc1.640033", running.width, running.height, running.fps, running.description),
            "width" to ConfigMessage(running.codec, 1920, running.height, running.fps, running.description),
            "height" to ConfigMessage(running.codec, running.width, 1080, running.fps, running.description),
            "fps" to ConfigMessage(running.codec, running.width, running.height, 120, running.description),
            "description" to ConfigMessage(running.codec, running.width, running.height, running.fps, changedDescription),
        )

        changes.forEach { (field, next) -> assertTrue(field, ConfigChange.needsNewDecoder(running, rendered = true, next = next)) }
    }
}
