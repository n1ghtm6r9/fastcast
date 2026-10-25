package dev.fastcast.client.update

import java.util.concurrent.TimeUnit

class UpdateCheckCache(
    private val nowMs: () -> Long = { TimeUnit.NANOSECONDS.toMillis(System.nanoTime()) },
) {

    private val fetchLock = Any()
    private var latest: Entry? = null

    fun resolve(fetch: () -> UpdateManifest?): UpdateManifest? {
        reusable()?.let { return it.manifest }
        synchronized(fetchLock) {
            reusable()?.let { return it.manifest }
            return fetch().also(::remember)
        }
    }

    fun dismiss() = remember(null)

    @Synchronized
    private fun reusable(): Entry? = latest?.takeIf { nowMs() - it.storedAtMs < REUSE_MS }

    @Synchronized
    private fun remember(manifest: UpdateManifest?) {
        latest = Entry(manifest, nowMs())
    }

    private class Entry(val manifest: UpdateManifest?, val storedAtMs: Long)

    companion object {
        const val REUSE_MS = 60_000L

        val shared = UpdateCheckCache()
    }
}
