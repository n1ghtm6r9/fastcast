package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Test

class TheBestDecodableCodecIsListedFirstTest {

    @Test
    fun `hevc comes before avc when both can be decoded`() {
        assertEquals(listOf("hevc", "avc"), SupportedCodecs.among { true })
    }

    @Test
    fun `avc alone is listed when hevc cannot be decoded`() {
        assertEquals(listOf("avc"), SupportedCodecs.among { mime -> mime == VideoMime.AVC })
    }

    @Test
    fun `hevc alone is listed when avc cannot be decoded`() {
        assertEquals(listOf("hevc"), SupportedCodecs.among { mime -> mime == VideoMime.HEVC })
    }

    @Test
    fun `nothing is listed when no codec can be decoded`() {
        assertEquals(emptyList<String>(), SupportedCodecs.among { false })
    }

    @Test
    fun `each codec is asked about by its mime type and hevc is asked first`() {
        val asked = mutableListOf<String>()

        SupportedCodecs.among { mime ->
            asked.add(mime)
            true
        }

        assertEquals(listOf("video/hevc", "video/avc"), asked)
    }
}
