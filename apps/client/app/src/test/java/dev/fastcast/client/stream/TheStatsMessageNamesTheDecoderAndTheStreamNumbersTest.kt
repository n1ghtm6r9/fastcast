package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Test

class TheStatsMessageNamesTheDecoderAndTheStreamNumbersTest {

    private val json = JSONObject(
        StatsMessage(
            decoder = "c2.mtk.hevc.decoder",
            mime = "video/hevc",
            width = 3840,
            height = 2160,
            fps = 59.9,
            mbps = 31.2,
            dropped = 4L,
            display = "3840x2160@60",
            receiveBuffer = 8_388_608,
            colorStandard = 1,
            colorRange = 2,
            colorTransfer = 3,
        ).toJson(),
    )

    @Test
    fun `the message is of type stats`() {
        assertEquals("stats", json.getString("type"))
    }

    @Test
    fun `the decoder and the mime type are named`() {
        assertEquals("c2.mtk.hevc.decoder", json.getString("decoder"))
        assertEquals("video/hevc", json.getString("mime"))
    }

    @Test
    fun `the picture size is given in pixels`() {
        assertEquals(3840, json.getInt("width"))
        assertEquals(2160, json.getInt("height"))
    }

    @Test
    fun `the rates are numbers`() {
        assertEquals(59.9, json.getDouble("fps"), 0.0)
        assertEquals(31.2, json.getDouble("mbps"), 0.0)
    }

    @Test
    fun `the dropped frame count is a whole number`() {
        assertEquals(4L, json.getLong("dropped"))
    }

    @Test
    fun `the active display mode is named`() {
        assertEquals("3840x2160@60", json.getString("display"))
    }

    @Test
    fun `the receive buffer of the link is given in bytes`() {
        assertEquals(8_388_608, json.getInt("receiveBuffer"))
    }

    @Test
    fun `the output colors are whole numbers`() {
        assertEquals(1, json.getInt("colorStandard"))
        assertEquals(2, json.getInt("colorRange"))
        assertEquals(3, json.getInt("colorTransfer"))
    }

    @Test
    fun `no other field is present`() {
        val expected = setOf(
            "type", "decoder", "mime", "width", "height", "fps", "mbps", "dropped", "display",
            "receiveBuffer", "colorStandard", "colorRange", "colorTransfer",
        )

        assertEquals(expected, json.keys().asSequence().toSet())
    }
}
