package dev.fastcast.client.debug

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheDebugMessagesFitTheServerReportLimitTest {

    private val manyFacts = List(100) { "ro.vendor.display.refresh.$it=1200000000" }

    private fun textsOf(messages: List<String>): List<String> =
        messages.flatMap { JSONObject(it).getString("text").split("; ") }

    private fun assertEachFits(messages: List<String>) {
        messages.forEach { assertTrue(it, it.toByteArray(Charsets.UTF_8).size <= DebugMessages.MAX_BYTES) }
    }

    @Test
    fun `short facts share one message`() {
        val messages = DebugMessages.pack("display", listOf("refresh 60.0", "mode 1"))

        assertEquals(1, messages.size)
        val json = JSONObject(messages.single())
        assertEquals("debug", json.getString("type"))
        assertEquals("display", json.getString("step"))
        assertEquals("1/1", json.getString("part"))
        assertEquals("refresh 60.0; mode 1", json.getString("text"))
    }

    @Test
    fun `many facts are spread over messages that each fit the limit`() {
        val messages = DebugMessages.pack("props", manyFacts)

        assertTrue(messages.size > 1)
        assertEachFits(messages)
        assertEquals(manyFacts, textsOf(messages))
    }

    @Test
    fun `the parts are numbered in order`() {
        val messages = DebugMessages.pack("props", manyFacts)

        val parts = messages.map { JSONObject(it).getString("part") }
        assertEquals(List(messages.size) { "${it + 1}/${messages.size}" }, parts)
    }

    @Test
    fun `a fact longer than one message is cut into consecutive pieces`() {
        val fact = (1..400).joinToString(",") { "mode$it" }

        val messages = DebugMessages.pack("display", listOf(fact))

        assertTrue(messages.size > 1)
        assertEachFits(messages)
        assertEquals(fact, messages.joinToString("") { JSONObject(it).getString("text") })
    }

    @Test
    fun `text outside ascii is measured in bytes`() {
        val facts = List(40) { "дисплей $it: частота 60 Гц" }

        val messages = DebugMessages.pack("display", facts)

        assertEachFits(messages)
        assertEquals(facts, textsOf(messages))
    }

    @Test
    fun `a step without facts still sends one message`() {
        val messages = DebugMessages.pack("done", emptyList())

        assertEquals(1, messages.size)
        assertEquals("", JSONObject(messages.single()).getString("text"))
    }
}
