package dev.fastcast.client.decoder

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import dev.fastcast.client.testing.RealHevcConfigurationRecord
import org.junit.Assert.assertArrayEquals
import org.junit.Assert.assertEquals
import org.junit.Assert.assertNull
import org.junit.Test

class TheConfigurationParserPicksTheRecordFormatByMimeTypeTest {

    private val startCode = byteArrayOf(0, 0, 0, 1)

    @Test
    fun `an avc description yields the sequence and the picture parameter set as two buffers`() {
        val configuration =
            requireNotNull(CodecConfigurationParser.parse(VideoMime.AVC, RealAvcConfigurationRecord.BYTES))

        assertEquals(2, configuration.csdBuffers.size)
        assertArrayEquals(startCode + RealAvcConfigurationRecord.SEQUENCE_PARAMETER_SET, configuration.csdBuffers[0])
        assertArrayEquals(startCode + RealAvcConfigurationRecord.PICTURE_PARAMETER_SET, configuration.csdBuffers[1])
    }

    @Test
    fun `an hevc description yields all three parameter sets as one buffer`() {
        val configuration =
            requireNotNull(CodecConfigurationParser.parse(VideoMime.HEVC, RealHevcConfigurationRecord.BYTES))
        val expected = startCode + RealHevcConfigurationRecord.VIDEO_PARAMETER_SET +
            startCode + RealHevcConfigurationRecord.SEQUENCE_PARAMETER_SET +
            startCode + RealHevcConfigurationRecord.PICTURE_PARAMETER_SET

        assertEquals(1, configuration.csdBuffers.size)
        assertArrayEquals(expected, configuration.csdBuffers.single())
    }

    @Test
    fun `both record formats report the nal length size of the stream`() {
        val avc = requireNotNull(CodecConfigurationParser.parse(VideoMime.AVC, RealAvcConfigurationRecord.BYTES))
        val hevc = requireNotNull(CodecConfigurationParser.parse(VideoMime.HEVC, RealHevcConfigurationRecord.BYTES))

        assertEquals(4, avc.nalLengthSize)
        assertEquals(4, hevc.nalLengthSize)
    }

    @Test
    fun `an hevc description is not accepted as avc`() {
        assertNull(CodecConfigurationParser.parse(VideoMime.AVC, RealHevcConfigurationRecord.BYTES))
    }

    @Test
    fun `an avc description is not accepted as hevc`() {
        assertNull(CodecConfigurationParser.parse(VideoMime.HEVC, RealAvcConfigurationRecord.BYTES))
    }

    @Test
    fun `an unknown mime type has no parser`() {
        assertNull(CodecConfigurationParser.parse("video/x-vnd.on2.vp9", RealAvcConfigurationRecord.BYTES))
        assertNull(CodecConfigurationParser.parse("", RealHevcConfigurationRecord.BYTES))
    }

    @Test
    fun `an empty description is rejected for both formats`() {
        assertNull(CodecConfigurationParser.parse(VideoMime.AVC, ByteArray(0)))
        assertNull(CodecConfigurationParser.parse(VideoMime.HEVC, ByteArray(0)))
    }
}
