package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheMedianTrendTellsHowAValueMovedTest {

    @Test
    fun `every full group adds its middle value`() {
        val trend = MedianTrend(groupSize = 3, groups = 4)
        listOf(5L, 1L, 3L, 10L, 30L, 20L).forEach(trend::add)

        assertEquals(listOf(3L, 20L), trend.medians())
    }

    @Test
    fun `a group still filling is left out`() {
        val trend = MedianTrend(groupSize = 3, groups = 4)
        listOf(5L, 1L, 3L, 7L, 9L).forEach(trend::add)

        assertEquals(listOf(3L), trend.medians())
    }

    @Test
    fun `only the newest groups are kept`() {
        val trend = MedianTrend(groupSize = 2, groups = 2)
        listOf(1L, 1L, 2L, 2L, 3L, 3L).forEach(trend::add)

        assertEquals(listOf(2L, 3L), trend.medians())
    }

    @Test
    fun `nothing added means no groups`() {
        assertEquals(emptyList<Long>(), MedianTrend(groupSize = 2, groups = 2).medians())
    }
}
