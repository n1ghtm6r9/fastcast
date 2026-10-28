package dev.fastcast.client.stream

import org.json.JSONObject
import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Test

class TheStatsMessageCarriesTheLinkCountersTest {

    private fun messageWith(
        socketDrops: Int? = null,
        inboxDrops: Int? = null,
        linkPauseMs: Int? = null,
    ): JSONObject = JSONObject(
        StatsMessage(
            decoder = "c2.mtk.hevc.decoder",
            mime = "video/hevc",
            width = 2560,
            height = 1440,
            fps = 119.8,
            mbps = 54.6,
            dropped = 0L,
            display = "2560x1440@120",
            socketDrops = socketDrops,
            inboxDrops = inboxDrops,
            linkPauseMs = linkPauseMs,
        ).toJson(),
    )

    @Test
    fun `socket drops the tv cannot count are left out`() {
        assertFalse(messageWith().has("socketDrops"))
    }

    @Test
    fun `counted socket drops are sent`() {
        assertEquals(417, messageWith(socketDrops = 417).getInt("socketDrops"))
    }

    @Test
    fun `a socket that dropped nothing still says so`() {
        assertEquals(0, messageWith(socketDrops = 0).getInt("socketDrops"))
    }

    @Test
    fun `inbox drops the tv cannot count are left out`() {
        assertFalse(messageWith().has("inboxDrops"))
    }

    @Test
    fun `counted inbox drops are sent`() {
        assertEquals(23, messageWith(inboxDrops = 23).getInt("inboxDrops"))
    }

    @Test
    fun `a link pause that was not measured is left out`() {
        assertFalse(messageWith().has("linkPauseMs"))
    }

    @Test
    fun `the longest link pause is sent in milliseconds`() {
        assertEquals(140, messageWith(linkPauseMs = 140).getInt("linkPauseMs"))
    }
}
