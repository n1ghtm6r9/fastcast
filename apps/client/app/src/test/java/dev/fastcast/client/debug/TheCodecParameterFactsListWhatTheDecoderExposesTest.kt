package dev.fastcast.client.debug

import org.junit.Assert.assertEquals
import org.junit.Test

class TheCodecParameterFactsListWhatTheDecoderExposesTest {

    @Test
    fun `vendor parameters are listed by name with their values`() {
        assertEquals(
            listOf("vendor parameters (2): vendor.a.value=?int, vendor.b.value=1", "output format: none"),
            CodecParameterFacts.of(mapOf("vendor.b.value" to "1", "vendor.a.value" to "?int"), emptyMap()),
        )
    }

    @Test
    fun `a decoder without vendor parameters says so`() {
        assertEquals(
            listOf("vendor parameters: none", "output format: width=3840"),
            CodecParameterFacts.of(emptyMap(), mapOf("width" to "3840")),
        )
    }

    @Test
    fun `the output format is listed by key without codec specific data`() {
        assertEquals(
            listOf("vendor parameters: none", "output format: height=2160, mime=video/hevc"),
            CodecParameterFacts.of(emptyMap(), mapOf("mime" to "video/hevc", "csd-0" to "bytes", "height" to "2160")),
        )
    }
}
