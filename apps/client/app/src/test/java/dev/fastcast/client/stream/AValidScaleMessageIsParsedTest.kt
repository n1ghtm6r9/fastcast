package dev.fastcast.client.stream

import org.junit.Assert.assertEquals
import org.junit.Test

class AValidScaleMessageIsParsedTest {

    private val scale = requireNotNull(ScaleMessage.parse("""{"type":"scale","percent":150,"min":100,"max":300}"""))

    @Test
    fun `the percent is read`() {
        assertEquals(150, scale.percent)
    }

    @Test
    fun `the smallest and the largest percent are read`() {
        assertEquals(100, scale.min)
        assertEquals(300, scale.max)
    }

    @Test
    fun `unknown extra fields are ignored`() {
        val text = """{"output":0,"type":"scale","percent":125,"min":100,"max":250}"""

        assertEquals(125, requireNotNull(ScaleMessage.parse(text)).percent)
    }
}
