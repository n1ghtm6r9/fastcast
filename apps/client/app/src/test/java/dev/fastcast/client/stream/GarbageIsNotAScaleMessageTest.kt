package dev.fastcast.client.stream

import org.junit.Assert.assertNull
import org.junit.Test

class GarbageIsNotAScaleMessageTest {

    @Test
    fun `text that is not json is rejected`() {
        listOf("", "   ", "garbage", "{", "}", "scale", "null", "[]", "150").forEach { text ->
            assertNull("\"$text\"", ScaleMessage.parse(text))
        }
    }

    @Test
    fun `json of another message type is rejected`() {
        assertNull(ScaleMessage.parse("""{"type":"config","percent":150,"min":100,"max":300}"""))
    }

    @Test
    fun `a scale missing a field is rejected`() {
        listOf(
            """{"type":"scale","min":100,"max":300}""",
            """{"type":"scale","percent":150,"max":300}""",
            """{"type":"scale","percent":150,"min":100}""",
        ).forEach { text -> assertNull(text, ScaleMessage.parse(text)) }
    }

    @Test
    fun `a percent that is not positive is rejected`() {
        listOf(
            """{"type":"scale","percent":0,"min":100,"max":300}""",
            """{"type":"scale","percent":-150,"min":100,"max":300}""",
            """{"type":"scale","percent":150,"min":0,"max":300}""",
            """{"type":"scale","percent":150,"min":100,"max":-300}""",
        ).forEach { text -> assertNull(text, ScaleMessage.parse(text)) }
    }
}
