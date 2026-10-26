package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertTrue
import org.junit.Test

class TheStatsMessageNeverOutgrowsOneDatagramTest {

    private fun messageWith(
        decoder: String = "c2.mtk.hevc.decoder",
        display: String = "3840x2160@60",
        error: String? = null,
    ): StatsMessage = StatsMessage(
        decoder = decoder,
        mime = "video/hevc",
        width = 3840,
        height = 2160,
        fps = 59.9,
        mbps = 31.2,
        dropped = 0L,
        display = display,
        receiveBuffer = 8_388_608,
        colorStandard = 1,
        colorRange = 2,
        colorTransfer = 3,
        error = error,
    )

    private fun sizeOf(text: String): Int = text.toByteArray(Charsets.UTF_8).size

    @Test
    fun `the limit is the payload of one datagram`() {
        assertEquals(1388, StatsMessage.MAX_BYTES)
    }

    @Test
    fun `a message of ordinary size is sent whole`() {
        val json = JSONObject(messageWith(error = "Decoder failed").toJson())

        assertEquals("Decoder failed", json.getString("error"))
        assertEquals("3840x2160@60", json.getString("display"))
    }

    @Test
    fun `a very long error is cut until the message fits`() {
        val error = "x".repeat(5_000)

        val text = messageWith(error = error).toJson()

        val kept = JSONObject(text).getString("error")
        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
        assertTrue(kept.isNotEmpty())
        assertTrue(error.startsWith(kept))
    }

    @Test
    fun `a cut error keeps as much text as fits`() {
        val text = messageWith(error = "x".repeat(5_000)).toJson()

        assertEquals(StatsMessage.MAX_BYTES, sizeOf(text))
    }

    @Test
    fun `the other fields survive when only the error is too long`() {
        val json = JSONObject(messageWith(error = "x".repeat(5_000)).toJson())

        assertEquals("stats", json.getString("type"))
        assertEquals("c2.mtk.hevc.decoder", json.getString("decoder"))
        assertEquals("video/hevc", json.getString("mime"))
        assertEquals("3840x2160@60", json.getString("display"))
        assertEquals(8_388_608, json.getInt("receiveBuffer"))
    }

    @Test
    fun `an error made of quotes still fits and stays valid`() {
        val text = messageWith(error = "\"".repeat(2_000)).toJson()

        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
        assertTrue(JSONObject(text).getString("error").all { it == '"' })
    }

    @Test
    fun `an error of characters outside the basic plane is never cut inside a character`() {
        val text = messageWith(error = "😀".repeat(800)).toJson()

        val kept = JSONObject(text).getString("error")
        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
        assertTrue(kept.isNotEmpty())
        assertEquals(kept, String(kept.toByteArray(Charsets.UTF_8), Charsets.UTF_8))
    }

    @Test
    fun `an error in a script that needs several bytes per letter fits`() {
        val text = messageWith(error = "ошибка декодера ".repeat(300)).toJson()

        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
    }

    @Test
    fun `a cut leaves no more than one wide character of room unused`() {
        val widest = 6

        val cyrillic = messageWith(error = "ошибка декодера ".repeat(300)).toJson()
        val emoji = messageWith(error = "😀".repeat(800)).toJson()
        val controls = messageWith(error = "\u0001".repeat(2_000)).toJson()

        assertTrue(sizeOf(cyrillic) > StatsMessage.MAX_BYTES - widest)
        assertTrue(sizeOf(emoji) > StatsMessage.MAX_BYTES - widest)
        assertTrue(sizeOf(controls) > StatsMessage.MAX_BYTES - widest)
        assertTrue(sizeOf(controls) <= StatsMessage.MAX_BYTES)
    }

    @Test
    fun `a label too long for the message is cut and the short error survives`() {
        val display = "d".repeat(3_000)

        val text = messageWith(display = display, error = "boom").toJson()

        val json = JSONObject(text)
        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
        assertEquals("boom", json.getString("error"))
        assertTrue(display.startsWith(json.getString("display")))
    }

    @Test
    fun `labels of any length leave a message that fits`() {
        val text = messageWith(decoder = "c".repeat(4_000), display = "d".repeat(4_000), error = "e".repeat(4_000)).toJson()

        assertTrue(sizeOf(text) <= StatsMessage.MAX_BYTES)
        assertEquals("stats", JSONObject(text).getString("type"))
    }
}
