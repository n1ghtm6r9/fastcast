package dev.fastcast.client.stream

import java.util.Base64

class ConfigMessage(
    val codec: String,
    val width: Int,
    val height: Int,
    val fps: Int,
    val description: ByteArray,
) {

    companion object {
        private const val TYPE = "type"
        private const val TYPE_CONFIG = "config"
        private const val CODEC = "codec"
        private const val WIDTH = "width"
        private const val HEIGHT = "height"
        private const val FPS = "fps"
        private const val DESCRIPTION = "description"

        fun parse(text: String): ConfigMessage? {
            val json = JsonObjects.parse(text) ?: return null
            if (json.optString(TYPE) != TYPE_CONFIG) return null
            val codec = json.optString(CODEC).takeIf { it.isNotBlank() } ?: return null
            val width = json.optInt(WIDTH).takeIf { it > 0 } ?: return null
            val height = json.optInt(HEIGHT).takeIf { it > 0 } ?: return null
            val fps = json.optInt(FPS).takeIf { it > 0 } ?: return null
            val description = decodeBase64(json.optString(DESCRIPTION)) ?: return null
            return ConfigMessage(codec, width, height, fps, description)
        }

        private fun decodeBase64(value: String): ByteArray? = try {
            Base64.getDecoder().decode(value).takeIf { it.isNotEmpty() }
        } catch (e: IllegalArgumentException) {
            null
        }
    }
}
