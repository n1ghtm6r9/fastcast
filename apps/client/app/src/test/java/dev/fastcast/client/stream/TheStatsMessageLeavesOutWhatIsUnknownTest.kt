package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TheStatsMessageLeavesOutWhatIsUnknownTest {

    private fun messageWith(
        receiveBuffer: Int? = null,
        colorStandard: Int? = null,
        colorRange: Int? = null,
        colorTransfer: Int? = null,
        error: String? = null,
        network: String? = null,
        delayMs: Double? = null,
        tunneled: Boolean = false,
    ): JSONObject = JSONObject(
        StatsMessage(
            decoder = "c2.mtk.hevc.decoder",
            mime = "video/hevc",
            width = 3840,
            height = 2160,
            fps = 59.9,
            mbps = 31.2,
            dropped = 0L,
            display = "3840x2160@60",
            receiveBuffer = receiveBuffer,
            colorStandard = colorStandard,
            colorRange = colorRange,
            colorTransfer = colorTransfer,
            error = error,
            network = network,
            delayMs = delayMs,
            tunneled = tunneled,
        ).toJson(),
    )

    @Test
    fun `a network the tv did not name is left out`() {
        assertFalse(messageWith().has("network"))
    }

    @Test
    fun `a named network is sent`() {
        assertEquals("ethernet 1000 Mbit/s", messageWith(network = "ethernet 1000 Mbit/s").getString("network"))
    }

    @Test
    fun `colors the decoder did not report are left out`() {
        val json = messageWith()

        assertFalse(json.has("colorStandard"))
        assertFalse(json.has("colorRange"))
        assertFalse(json.has("colorTransfer"))
    }

    @Test
    fun `only the colors that are known are present`() {
        val json = messageWith(colorStandard = 1, colorTransfer = 3)

        assertEquals(1, json.getInt("colorStandard"))
        assertFalse(json.has("colorRange"))
        assertEquals(3, json.getInt("colorTransfer"))
    }

    @Test
    fun `a color of zero is still a color`() {
        val json = messageWith(colorStandard = 0, colorRange = 0, colorTransfer = 0)

        assertTrue(json.has("colorStandard"))
        assertTrue(json.has("colorRange"))
        assertTrue(json.has("colorTransfer"))
    }

    @Test
    fun `a receive buffer that is not known is left out`() {
        assertFalse(messageWith().has("receiveBuffer"))
    }

    @Test
    fun `a known receive buffer is present`() {
        assertEquals(8_388_608, messageWith(receiveBuffer = 8_388_608).getInt("receiveBuffer"))
    }

    @Test
    fun `a delay the tv did not measure is left out`() {
        assertFalse(messageWith().has("delayMs"))
    }

    @Test
    fun `a measured delay is sent in tenths of a millisecond`() {
        assertEquals(57.3, messageWith(delayMs = 57.26).getDouble("delayMs"), 0.0)
    }

    @Test
    fun `a stream on the usual path leaves the video path flag out`() {
        assertFalse(messageWith().has("tunneled"))
    }

    @Test
    fun `a stream through the tv video path says so`() {
        assertTrue(messageWith(tunneled = true).getBoolean("tunneled"))
    }

    @Test
    fun `a plain report has no error field`() {
        assertFalse(messageWith().has("error"))
    }

    @Test
    fun `the unknown colors are left out of a failure report too`() {
        val json = messageWith(error = "boom")

        assertFalse(json.has("colorStandard"))
        assertEquals("boom", json.getString("error"))
    }
}
