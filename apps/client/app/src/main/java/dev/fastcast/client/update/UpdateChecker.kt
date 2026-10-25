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
        val manifest = UpdateManifest.parse(fetchText()) ?: throw IOException("The update manifest is unusable")
        return manifest.takeIf { it.versionCode > installedVersionCode }
    }

    private fun fetchText(): String = client.newCall(manifestRequest()).execute().use(::readCappedBody)

    private fun manifestRequest(): Request = try {
        Request.Builder().url(manifestUrl).build()
    } catch (e: IllegalArgumentException) {
        throw IOException("The update manifest URL is invalid", e)
    }

    private fun readCappedBody(response: Response): String {
        if (!response.isSuccessful) throw IOException("The update manifest request failed with HTTP ${response.code}")
        val source = response.body?.source() ?: throw IOException("The update manifest response has no body")
        source.request(MAX_BODY_BYTES + 1)
        if (source.buffer.size > MAX_BODY_BYTES) throw IOException("The update manifest exceeds $MAX_BODY_BYTES bytes")
        return source.buffer.readUtf8()
    }

    private companion object {
        const val MAX_BODY_BYTES = 16L * 1024
    }
}
