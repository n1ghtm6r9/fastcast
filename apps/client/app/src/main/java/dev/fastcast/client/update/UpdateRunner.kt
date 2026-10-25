package dev.fastcast.client.update

import java.io.File
import java.util.concurrent.Executor

class UpdateRunner(
    private val listener: UpdateListener,
    private val deliver: Executor,
    private val cache: UpdateCheckCache,
    private val rejected: RejectedVersions,
    private val fetch: () -> UpdateManifest?,
    private val download: (UpdateManifest, (Long, Long) -> Unit, () -> Boolean) -> File,
    private val install: (File, UpdateManifest) -> Unit,
) {

    private val flowLock = Any()
    private var current: Flow? = null

    @Synchronized
    fun start() {
        if (current != null) return
        current = Flow().also(Flow::begin)
    }

    @Synchronized
    fun stop() {
        current?.end()
        current = null
    }

    private inner class Flow : Runnable {

        private val thread = Thread(this, THREAD_NAME).apply { isDaemon = true }
        private var lastPercent = NO_PERCENT

        @Volatile
        private var active = true

        fun begin() = thread.start()

        fun end() {
            active = false
        }

        override fun run() {
            synchronized(flowLock) {
                if (!active) return
                try {
                    update()
                } catch (e: Exception) {
                    if (active) fail(e)
                }
            }
        }

        private fun update() {
            val manifest = cache.resolve(fetch)?.takeUnless { rejected.isRejected(it.versionCode) }
            if (!active) return
            if (manifest == null) {
                report { onNoUpdate() }
                return
            }
            report { onUpdateFound(manifest) }
            try {
                downloadAndInstall(manifest)
            } catch (e: Exception) {
                if (active) cache.dismiss()
                throw e
            }
        }

        private fun downloadAndInstall(manifest: UpdateManifest) {
            val apk = download(manifest, ::reportProgress, ::isStopped)
            try {
                if (active) install(apk, manifest)
            } catch (e: Exception) {
                rejected.reject(manifest.versionCode)
                throw e
            } finally {
                apk.delete()
            }
        }

        private fun isStopped(): Boolean = !active

        private fun reportProgress(received: Long, total: Long) {
            val percent = (received * PERCENT_SCALE / total).toInt().coerceIn(0, PERCENT_SCALE)
            if (percent == lastPercent) return
            lastPercent = percent
            report { onUpdateProgress(percent) }
        }

        private fun fail(cause: Exception) {
            report { onUpdateFailed(cause) }
        }

        private fun report(event: UpdateListener.() -> Unit) {
            deliver.execute { if (active) listener.event() }
        }
    }

    private companion object {
        const val THREAD_NAME = "fastcast-update"
        const val PERCENT_SCALE = 100
        const val NO_PERCENT = -1
    }
}
