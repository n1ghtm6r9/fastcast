package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Test

class AFailureReportCarriesTheErrorTextTest {

    private fun failureWith(error: String): JSONObject = JSONObject(
        StatsMessage(
            decoder = "c2.mtk.hevc.decoder",
            mime = "video/hevc",
            width = 3840,
            height = 2160,
            fps = 0.0,
            mbps = 0.0,
            dropped = 12L,
            display = "3840x2160@60",
            colorStandard = 1,
            colorRange = 2,
            colorTransfer = 3,
            error = error,
        ).toJson(),
    )

    @Test
    fun `the error is sent next to the usual fields`() {
        val json = failureWith("Decoder failed")

        assertEquals("stats", json.getString("type"))
        assertEquals("Decoder failed", json.getString("error"))
        assertEquals("c2.mtk.hevc.decoder", json.getString("decoder"))
        assertEquals(12L, json.getLong("dropped"))
    }

    @Test
    fun `quotes and line breaks in the error survive the round trip`() {
        val error = "CodecException: \"configure\" failed\nat line 1\tof C:\\codec"

        assertEquals(error, failureWith(error).getString("error"))
    }

    @Test
    fun `text outside ascii in the error survives the round trip`() {
        val error = "Ошибка декодера ☃"

        assertEquals(error, failureWith(error).getString("error"))
    }

    @Test
    fun `an empty error is still reported as a failure`() {
        assertEquals("", failureWith("").getString("error"))
    }
}
