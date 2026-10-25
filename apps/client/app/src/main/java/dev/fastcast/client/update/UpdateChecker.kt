package dev.fastcast.client.update

import dev.fastcast.client.BuildConfig
import java.io.IOException
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.Response

class UpdateChecker(
    private val client: OkHttpClient = UpdateHttpClient.shared,
    private val manifestUrl: String = BuildConfig.UPDATE_MANIFEST_URL,
    private val installedVersionCode: Int = BuildConfig.VERSION_CODE,
) {

    fun check(): UpdateManifest? {
        val text = fetchText() ?: return null
        return UpdateManifest.parse(text)?.takeIf { it.versionCode > installedVersionCode }
    }

    private fun fetchText(): String? = try {
        client.newCall(Request.Builder().url(manifestUrl).build()).execute().use(::readCappedBody)
    } catch (e: IOException) {
        null
    } catch (e: IllegalArgumentException) {
        null
    }

    private fun readCappedBody(response: Response): String? {
        if (!response.isSuccessful) return null
        val source = response.body?.source() ?: return null
        source.request(MAX_BODY_BYTES + 1)
        if (source.buffer.size > MAX_BODY_BYTES) return null
        return source.buffer.readUtf8()
    }

    private companion object {
        const val MAX_BODY_BYTES = 16L * 1024
    }
}
