package dev.fastcast.client.stream

import org.junit.Assert.assertNull
import org.junit.Test

class GarbageIsNotAConfigMessageTest {

    @Test
    fun `text that is not json is rejected`() {
        listOf("", "   ", "garbage", "{", "}", "config", "null", "[]", "123").forEach { text ->
            assertNull("\"$text\"", ConfigMessage.parse(text))
        }
    }

    @Test
    fun `an empty object is rejected`() {
        assertNull(ConfigMessage.parse("{}"))
    }

    @Test
    fun `json of another message type is rejected`() {
        val text = """{"type":"stats","codec":"avc1.64002a","width":1,"height":1,"fps":1,"description":"AQ=="}"""

        assertNull(ConfigMessage.parse(text))
    }

    @Test
    fun `a description that is not base64 is rejected`() {
        val text = """{"type":"config","codec":"avc1.64002a","width":1,"height":1,"fps":1,"description":"%%%"}"""

        assertNull(ConfigMessage.parse(text))
    }

    @Test
    fun `a size or rate that is not positive is rejected`() {
        val template = """{"type":"config","codec":"avc1.64002a","width":W,"height":H,"fps":F,"description":"AQ=="}"""

        listOf(
            Triple(0, 720, 60),
            Triple(1280, 0, 60),
            Triple(1280, 720, 0),
            Triple(-1280, 720, 60),
            Triple(1280, -720, 60),
            Triple(1280, 720, -60),
        ).forEach { (width, height, fps) ->
            val text = template.replace("W", "$width").replace("H", "$height").replace("F", "$fps")

            assertNull(text, ConfigMessage.parse(text))
        }
    }
}
