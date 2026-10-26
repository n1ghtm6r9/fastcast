package dev.fastcast.client.decoder

import android.media.MediaCodecInfo.CodecProfileLevel
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class TenBitAv1IsRecognisedByItsMainTenProfilesTest {

    @Test
    fun `every main 10 av1 profile is ten bit`() {
        val mainTenProfiles = listOf(
            CodecProfileLevel.AV1ProfileMain10,
            CodecProfileLevel.AV1ProfileMain10HDR10,
            CodecProfileLevel.AV1ProfileMain10HDR10Plus,
        )

        mainTenProfiles.forEach { profile ->
            val profiles = listOf(CodecProfileLevel.AV1ProfileMain8, profile)

            assertTrue("profile $profile", TenBitProfiles.includes(VideoMime.AV1, profiles))
        }
    }

    @Test
    fun `plain main av1 is not ten bit`() {
        assertFalse(TenBitProfiles.includes(VideoMime.AV1, listOf(CodecProfileLevel.AV1ProfileMain8)))
    }
}
