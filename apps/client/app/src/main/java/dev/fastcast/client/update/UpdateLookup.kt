package dev.fastcast.client.update

import java.util.concurrent.Executor

class UpdateLookup(
    private val cache: UpdateCheckCache,
    private val fetch: () -> UpdateManifest?,
    private val deliver: Executor,
    private val onUpdateAvailable: () -> Unit,
) {

    private var current: Search? = null

    @Synchronized
    fun start() {
        if (current != null) return
        current = Search().also(Search::begin)
    }

    @Synchronized
    fun stop() {
        current?.end()
        current = null
    }

    private inner class Search : Runnable {

        private val thread = Thread(this, THREAD_NAME).apply { isDaemon = true }

        @Volatile
        private var active = true

        fun begin() = thread.start()

        fun end() {
            active = false
        }

        override fun run() {
            val manifest = try {
                cache.resolve(fetch)
            } catch (e: Exception) {
                null
            }
            if (manifest != null) deliver.execute { if (active) onUpdateAvailable() }
        }
    }

    private companion object {
        const val THREAD_NAME = "fastcast-update-lookup"
    }
}
