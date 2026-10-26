package dev.fastcast.client.decoder

import android.media.MediaCodecInfo.CodecProfileLevel
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TenBitHevcIsRecognisedByItsMainTenProfilesTest {

    @Test
    fun `every main 10 hevc profile is ten bit`() {
        val mainTenProfiles = listOf(
            CodecProfileLevel.HEVCProfileMain10,
            CodecProfileLevel.HEVCProfileMain10HDR10,
            CodecProfileLevel.HEVCProfileMain10HDR10Plus,
        )

        mainTenProfiles.forEach { profile ->
            val profiles = listOf(CodecProfileLevel.HEVCProfileMain, profile)

            assertTrue("profile $profile", TenBitProfiles.includes(VideoMime.HEVC, profiles))
        }
    }

    @Test
    fun `plain main hevc is not ten bit`() {
        val profiles = listOf(CodecProfileLevel.HEVCProfileMain, CodecProfileLevel.HEVCProfileMainStill)

        assertFalse(TenBitProfiles.includes(VideoMime.HEVC, profiles))
    }

    @Test
    fun `avc is never ten bit`() {
        assertFalse(TenBitProfiles.includes(VideoMime.AVC, listOf(CodecProfileLevel.AVCProfileHigh10)))
    }

    @Test
    fun `a decoder without profiles is not ten bit`() {
        assertFalse(TenBitProfiles.includes(VideoMime.HEVC, emptyList()))
    }
}
