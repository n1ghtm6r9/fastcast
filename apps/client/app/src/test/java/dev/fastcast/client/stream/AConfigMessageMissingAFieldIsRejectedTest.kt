package dev.fastcast.client.stream

import dev.fastcast.client.testing.RealAvcConfigurationRecord
import java.util.Base64
import org.json.JSONObject
import org.junit.Assert.assertNotNull
import org.junit.Assert.assertNull
import org.junit.Test

class AConfigMessageMissingAFieldIsRejectedTest {

    private fun validJson(): JSONObject = JSONObject()
        .put("type", "config")
        .put("codec", "avc1.64002a")
        .put("width", 2560)
        .put("height", 1440)
        .put("fps", 60)
        .put("description", Base64.getEncoder().encodeToString(RealAvcConfigurationRecord.BYTES))

    @Test
    fun `the complete message is accepted`() {
        assertNotNull(ConfigMessage.parse(validJson().toString()))
    }

    @Test
    fun `removing any single field makes the message invalid`() {
        listOf("type", "codec", "width", "height", "fps", "description").forEach { field ->
            val json = validJson().apply { remove(field) }

            assertNull("without $field", ConfigMessage.parse(json.toString()))
        }
    }

    @Test
    fun `blanking the codec makes the message invalid`() {
        val json = validJson().put("codec", "  ")

        assertNull(ConfigMessage.parse(json.toString()))
    }

    @Test
    fun `an empty description makes the message invalid`() {
        val json = validJson().put("description", "")

        assertNull(ConfigMessage.parse(json.toString()))
    }
}
