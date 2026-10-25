package dev.fastcast.client.update

import java.io.File
import java.io.FileOutputStream
import java.io.IOException
import java.io.InputStream
import java.security.MessageDigest
import okhttp3.OkHttpClient
import okhttp3.Request

class UpdateDownloader(
    private val directory: File,
    private val client: OkHttpClient = UpdateHttpClient.shared,
) {

    fun download(
        manifest: UpdateManifest,
        onProgress: (Long, Long) -> Unit,
        isCancelled: () -> Boolean = { false },
    ): File {
        clearDirectory()
        val target = File(directory, "fastcast-tv-${manifest.versionCode}.apk")
        return try {
            fetch(manifest, target, onProgress, isCancelled)
            target
        } catch (e: Exception) {
            target.delete()
            throw e
        }
    }

    private fun clearDirectory() {
        directory.mkdirs()
        directory.listFiles()?.forEach(File::delete)
    }

    private fun fetch(
        manifest: UpdateManifest,
        target: File,
        onProgress: (Long, Long) -> Unit,
        isCancelled: () -> Boolean,
    ) {
        val request = Request.Builder().url(manifest.url).build()
        client.newCall(request).execute().use { response ->
            if (!response.isSuccessful) throw IOException("Download failed with HTTP ${response.code}")
            val body = response.body ?: throw IOException("Download has no body")
            save(body.byteStream(), target, manifest, onProgress, isCancelled)
        }
    }

    private fun save(
        input: InputStream,
        target: File,
        manifest: UpdateManifest,
        onProgress: (Long, Long) -> Unit,
        isCancelled: () -> Boolean,
    ) {
        val digest = MessageDigest.getInstance(DIGEST_ALGORITHM)
        val buffer = ByteArray(BUFFER_BYTES)
        var received = 0L
        FileOutputStream(target).use { output ->
            while (true) {
                if (isCancelled()) throw IOException("Download cancelled")
                val count = input.read(buffer)
                if (count < 0) break
                received += count
                if (received > manifest.size) throw IOException("Download exceeds the announced ${manifest.size} bytes")
                digest.update(buffer, 0, count)
                output.write(buffer, 0, count)
                onProgress(received, manifest.size)
            }
        }
        if (received != manifest.size) throw IOException("Download is $received bytes instead of ${manifest.size}")
        if (!digest.digest().toLowerHex().equals(manifest.sha256, ignoreCase = true)) {
            throw IOException("Download does not match the announced SHA-256")
        }
    }

    private companion object {
        const val DIGEST_ALGORITHM = "SHA-256"
        const val BUFFER_BYTES = 16 * 1024
    }
}
