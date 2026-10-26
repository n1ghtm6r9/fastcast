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
        assertArrayEquals(IntArray(0), HelloCodecs.of(listOf("vp9", "vvc")))
    }

    @Test
    fun `av1 is wire id three and keeps its place in the order`() {
        val wire = HelloCodecs.of(listOf("av1", "hevc", "avc"), setOf("av1", "hevc"))

        assertArrayEquals(intArrayOf(3, 10, 2, 10, 1, 8), wire)
    }

    @Test
    fun `a ten bit codec is offered with ten bits and the others with eight`() {
        assertArrayEquals(intArrayOf(2, 10, 1, 8), HelloCodecs.of(listOf("hevc", "avc"), setOf("hevc")))
    }
}
