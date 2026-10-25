package dev.fastcast.client.update

import okhttp3.HttpUrl.Companion.toHttpUrlOrNull
import org.json.JSONException
import org.json.JSONObject

data class UpdateManifest(
    val versionCode: Int,
    val versionName: String,
    val url: String,
    val sha256: String,
    val size: Long,
) {

    companion object {
        private const val VERSION_CODE = "versionCode"
        private const val VERSION_NAME = "versionName"
        private const val URL = "url"
        private const val SHA256 = "sha256"
        private const val SIZE = "size"
        private val SHA256_PATTERN = Regex("[0-9a-fA-F]{64}")

        fun parse(text: String): UpdateManifest? {
            val json = parseJson(text) ?: return null
            val versionCode = json.integer(VERSION_CODE)?.takeIf { it in 1L..Int.MAX_VALUE.toLong() } ?: return null
            val versionName = json.text(VERSION_NAME)?.takeIf { it.isNotBlank() } ?: return null
            val url = json.text(URL)?.takeIf(::isHttps) ?: return null
            val sha256 = json.text(SHA256)?.takeIf { SHA256_PATTERN.matches(it) } ?: return null
            val size = json.integer(SIZE)?.takeIf { it > 0 } ?: return null
            return UpdateManifest(versionCode.toInt(), versionName, url, sha256.lowercase(), size)
        }

        private fun parseJson(text: String): JSONObject? = try {
            JSONObject(text)
        } catch (e: JSONException) {
            null
        }

        private fun JSONObject.text(key: String): String? = opt(key) as? String

        private fun JSONObject.integer(key: String): Long? = when (val value = opt(key)) {
            is Int -> value.toLong()
            is Long -> value
            else -> null
        }

        private fun isHttps(url: String): Boolean = url.toHttpUrlOrNull()?.isHttps == true
    }
}
