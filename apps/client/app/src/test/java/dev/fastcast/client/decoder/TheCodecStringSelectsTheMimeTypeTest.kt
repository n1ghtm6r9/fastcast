package dev.fastcast.client.decoder

import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheCodecStringSelectsTheMimeTypeTest {

    @Test
    fun `avc1 and avc3 codecs are h264 video`() {
        assertEquals("video/avc", VideoMime.forCodec("avc1.64002a"))
        assertEquals("video/avc", VideoMime.forCodec("avc3.640028"))
    }

    @Test
    fun `hvc1 and hev1 codecs are hevc video`() {
        assertEquals("video/hevc", VideoMime.forCodec("hvc1.1.6.L93.B0"))
        assertEquals("video/hevc", VideoMime.forCodec("hev1.1.6.L93.B0"))
    }

    @Test
    fun `av01 codecs are av1 video`() {
        assertEquals("video/av01", VideoMime.forCodec("av01.0.14M.10"))
        assertEquals("video/av01", VideoMime.forCodec("av01.0.08M.08"))
    }

    @Test
    fun `the prefix is matched without regard to case`() {
        assertEquals(VideoMime.AVC, VideoMime.forCodec("AVC1.64002A"))
        assertEquals(VideoMime.HEVC, VideoMime.forCodec("HVC1.1.6.L93.B0"))
        assertEquals(VideoMime.AV1, VideoMime.forCodec("AV01.0.14M.10"))
    }

    @Test
    fun `other codecs have no mime type`() {
        listOf("", "vp09.00.10.08", "mp4a.40.2", "xavc1.64002a", "xav01.0.14M.10").forEach { codec ->
            assertNull("\"$codec\"", VideoMime.forCodec(codec))
        }
    }
}
