package dev.fastcast.client.stream

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import java.util.Base64
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Test

class AValidConfigMessageIsParsedTest {

    private val description = Base64.getEncoder().encodeToString(RealAvcConfigurationRecord.BYTES)
    private val text = """
        {"type":"config","codec":"avc1.64002a","width":2560,"height":1440,"fps":60,"description":"$description"}
    """.trimIndent()
    private val config = requireNotNull(ConfigMessage.parse(text))

    @Test
    fun `the codec string is kept as sent`() {
        assertEquals("avc1.64002a", config.codec)
    }

    @Test
    fun `the picture size is read`() {
        assertEquals(2560, config.width)
        assertEquals(1440, config.height)
    }

    @Test
    fun `the frame rate is read`() {
        assertEquals(60, config.fps)
    }

    @Test
    fun `the description is the decoded avc configuration record`() {
        assertArrayEquals(RealAvcConfigurationRecord.BYTES, config.description)
    }

    @Test
    fun `unknown extra fields are ignored`() {
        val withExtra = text.replace("\"type\"", "\"bitrate\":50000000,\"type\"")

        assertEquals(2560, requireNotNull(ConfigMessage.parse(withExtra)).width)
    }
}
