package dev.fastcast.client.debug

interface DebugHost {

    fun factSections(): List<FactSection>

    fun measureVsync(durationMs: Long, done: (LongArray) -> Unit)

    fun askRefreshRate(rate: Float)

    fun clearRefreshRate()

    fun displayNow(): String

    fun isPlaying(): Boolean

    fun send(message: String): Boolean

    fun counters(): DebugCounters?

    fun presentedSince(startNanos: Long): LongArray

    fun streamFast(): String

    fun streamUsual()

    fun showProgress(step: DebugStep)

    fun showDone(result: FastStreamResult?)
}
