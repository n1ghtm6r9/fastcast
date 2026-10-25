package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Test

class TheStatsMessageRoundsTheRatesToTenthsTest {

    private fun messageWith(fps: Double, mbps: Double): JSONObject = JSONObject(
        StatsMessage(
            decoder = "c2.mtk.hevc.decoder",
            mime = "video/hevc",
            width = 3840,
            height = 2160,
            fps = fps,
            mbps = mbps,
            dropped = 0L,
            display = "3840x2160@60",
        ).toJson(),
    )

    @Test
    fun `the frame rate is rounded to one decimal`() {
        assertEquals(59.9, messageWith(fps = 59.93, mbps = 0.0).getDouble("fps"), 0.0)
        assertEquals(60.0, messageWith(fps = 59.96, mbps = 0.0).getDouble("fps"), 0.0)
        assertEquals(119.9, messageWith(fps = 119.88, mbps = 0.0).getDouble("fps"), 0.0)
    }

    @Test
    fun `the bit rate is rounded to one decimal`() {
        assertEquals(31.2, messageWith(fps = 0.0, mbps = 31.24).getDouble("mbps"), 0.0)
        assertEquals(31.3, messageWith(fps = 0.0, mbps = 31.26).getDouble("mbps"), 0.0)
        assertEquals(1234.6, messageWith(fps = 0.0, mbps = 1234.56).getDouble("mbps"), 0.0)
    }

    @Test
    fun `a rate that is nearly zero becomes zero`() {
        val json = messageWith(fps = 0.04, mbps = 0.03)

        assertEquals(0.0, json.getDouble("fps"), 0.0)
        assertEquals(0.0, json.getDouble("mbps"), 0.0)
    }

    @Test
    fun `an idle stream reports zero rates`() {
        val json = messageWith(fps = 0.0, mbps = 0.0)

        assertEquals(0.0, json.getDouble("fps"), 0.0)
        assertEquals(0.0, json.getDouble("mbps"), 0.0)
    }
}
