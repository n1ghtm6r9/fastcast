package dev.fastcast.client.stream

import org.junit.Assert.assertArrayEquals
import org.junit.Test

class HelloCodecsAreTheWireIdsInPreferenceOrderTest {

    @Test
    fun `known codecs keep the order they were named in and unknown ones are skipped`() {
        assertArrayEquals(intArrayOf(2, 8, 1, 8), HelloCodecs.of(listOf("hevc", "avc", "vp9")))
    }

    @Test
    fun `avc alone is its wire id with eight bits`() {
        assertArrayEquals(intArrayOf(1, 8), HelloCodecs.of(listOf("avc")))
    }

    @Test
    fun `no names give no codecs`() {
        assertArrayEquals(IntArray(0), HelloCodecs.of(emptyList()))
    }

    @Test
    fun `only unknown names give no codecs`() {
        assertArrayEquals(IntArray(0), HelloCodecs.of(listOf("vp9", "av1")))
    }
}
