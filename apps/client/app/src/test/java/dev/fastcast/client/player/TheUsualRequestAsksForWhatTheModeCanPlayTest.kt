package dev.fastcast.client.player

import dev.fastcast.client.display.DisplayModeOption
import org.junit.Assert.assertEquals
import org.junit.Test

class TheUsualRequestAsksForWhatTheModeCanPlayTest {

    private val decodable = { _: DisplayModeOption -> listOf("av1", "hevc", "avc") }
    private val tenBit = { _: DisplayModeOption -> setOf("av1", "hevc") }

    @Test
    fun `a tunneled mode asks for hevc in 10 bits when the decoder has a 10-bit profile`() {
        val request = UsualRequest.of(DisplayModeOption(1, 2560, 1440, 120f, tunneled = true), decodable, tenBit)

        assertEquals(listOf(2560, 1440, 120), listOf(request.width, request.height, request.fps))
        assertEquals(listOf("hevc"), request.codecs)
        assertEquals(setOf("hevc"), request.tenBitCodecs)
    }

    @Test
    fun `a tunneled mode asks for 8 bits when the decoder has no 10-bit profile`() {
        val request = UsualRequest.of(DisplayModeOption(1, 2560, 1440, 120f, tunneled = true), decodable) { emptySet() }

        assertEquals(emptySet<String>(), request.tenBitCodecs)
    }

    @Test
    fun `a usual mode asks for every decodable codec and the ones it plays in 10 bits`() {
        val request = UsualRequest.of(DisplayModeOption(1, 3840, 2160, 60f), decodable, tenBit)

        assertEquals(listOf(3840, 2160, 60), listOf(request.width, request.height, request.fps))
        assertEquals(listOf("av1", "hevc", "avc"), request.codecs)
        assertEquals(setOf("av1", "hevc"), request.tenBitCodecs)
    }
}
